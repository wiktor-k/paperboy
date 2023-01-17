use actix_web::{
    dev::Server, error::UrlGenerationError, web, web::Buf, App, HttpRequest, HttpResponse,
    HttpServer, Responder,
};

use service_binding::Listener;

mod file_url;
mod issue;
mod pdf_merger;
mod title;

use file_url::FileUrl;

#[derive(Debug, Clone)]
pub struct Params {
    pub issue_url: url::Url,

    pub collection_url: url::Url,

    pub client: reqwest::Client,

    pub scope: String,
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Could not parse URL")]
    Parse(#[from] url::ParseError),

    #[error("Request error")]
    Request(#[from] reqwest::Error),

    #[error("XML parsing error")]
    XmlParse(#[from] serde_xml_rs::Error),

    #[error("I/O error")]
    Io(#[from] std::io::Error),

    #[error("Cannot generate URL")]
    UrlGeneration(#[from] UrlGenerationError),
}

impl actix_web::error::ResponseError for Error {}

async fn print_issue(
    id: web::Path<u32>,
    params: web::Data<Params>,
) -> Result<impl Responder, Error> {
    let url = &params.issue_url;
    let base_url = url.clone().set_base_xml(*id);
    let resp = params.client.get(base_url).send().await?.bytes().await?;
    let issue: issue::Issue = serde_xml_rs::from_reader(resp.reader())?;
    let mut pdf = pdf_merger::Pdf::new();
    for page in issue.document.pages {
        let page_url = url.clone().set_file_name(*id, &page.file_name);
        let resp = params.client.get(page_url).send().await?.bytes().await?;
        pdf.append(resp.reader())?;
    }

    let mut merged = vec![];
    pdf.write_to(&mut merged)?;

    Ok(HttpResponse::Ok()
        .insert_header(("Content-Type", "application/pdf"))
        .insert_header(("Content-Disposition", "inline"))
        .body(merged))
}

async fn latest_issue(
    req: HttpRequest,
    id: web::Path<u32>,
    params: web::Data<Params>,
) -> Result<impl Responder, Error> {
    let mut url = params.collection_url.clone();
    url.query_pairs_mut()
        .append_pair("id_title", &format!("{}", id));

    let resp = params.client.get(url).send().await?.text().await?;

    if let Some(id) = title::find_latest_issue(resp) {
        let url = req.url_for("issue", [format!("{}", id)])?;
        Ok(HttpResponse::Found()
            .insert_header(("Location", url.as_str()))
            .body(""))
    } else {
        Ok(HttpResponse::NotFound().body(""))
    }
}

pub fn start(listener: Listener, params: Params) -> std::io::Result<Server> {
    let server = HttpServer::new(move || {
        App::new()
            .route("/healthz", web::get().to(HttpResponse::Ok))
            .service(
                web::scope(&params.scope)
                    .app_data(web::Data::new(params.clone()))
                    .route("/titles/{id}/issues/latest", web::get().to(latest_issue))
                    .service(web::resource("/issues/{id}").name("issue").to(print_issue)),
            )
    });

    let server = match listener {
        Listener::Unix(listener) => server.listen_uds(listener),
        Listener::Tcp(listener) => server.listen(listener),
    }?;

    Ok(server.run())
}
