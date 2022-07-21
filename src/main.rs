use clap::Parser;
use newspaper_fetcher::Params;
use service_binding::Binding;

#[derive(Debug, Parser)]
struct Args {
    #[clap(
        short = 'H',
        long,
        env = "HOST",
        default_value = "tcp://127.0.0.1:8080"
    )]
    host: Binding,

    #[clap(long, env = "ISSUE_URL")]
    issue_url: url::Url,

    #[clap(long, env = "COLLECTION_URL")]
    collection_url: url::Url,

    #[clap(long, env = "ADDITIONAL_TRUST_ROOT")]
    additional_trust_root: Option<std::path::PathBuf>,

    #[clap(long, env = "SCOPE", default_value = "")]
    scope: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let args = Args::parse();
    newspaper_fetcher::start(
        args.host.try_into()?,
        Params {
            issue_url: args.issue_url,
            collection_url: args.collection_url,
            scope: args.scope,
            client: {
                let additional_trust_root = args.additional_trust_root.and_then(|file_name| {
                    let mut buf = Vec::new();
                    use std::io::Read;
                    std::fs::File::open(file_name)
                        .ok()?
                        .read_to_end(&mut buf)
                        .ok()?;
                    reqwest::Certificate::from_der(&buf).ok()
                });

                let mut builder = reqwest::Client::builder();
                if let Some(trust_root) = additional_trust_root {
                    builder = builder.add_root_certificate(trust_root);
                }
                builder.build().unwrap()
            },
        },
    )?
    .await
}
