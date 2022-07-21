use url::Url;

pub trait FileUrl {
    fn set_base_xml(self, id: u32) -> Self;
    fn set_file_name(self, id: u32, file_name: &str) -> Self;
}

impl FileUrl for Url {
    fn set_base_xml(mut self: Url, id: u32) -> Url {
        {
            let mut query = self.query_pairs_mut();
            query.clear();
            query.append_pair("file", "/base.xml");
            query.append_pair("id_issue", &id.to_string());
            query.append_pair("page", "file");
        }
        self
    }

    fn set_file_name(mut self: Url, id: u32, file_name: &str) -> Url {
        {
            let mut query = self.query_pairs_mut();
            query.clear();
            query.append_pair("id_issue", &id.to_string());
            query.append_pair("page", "file");
            query.append_pair("deviceType", "1");
            query.append_pair("file", &format!("/{}", file_name));
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setting_base() {
        let url = Url::parse("https://example.com/test")
            .unwrap()
            .set_base_xml(123);
        assert_eq!(
            url.to_string(),
            "https://example.com/test?file=%2Fbase.xml&id_issue=123&page=file"
        );
    }

    #[test]
    fn setting_file_url() {
        let url = Url::parse("https://example.com/test")
            .unwrap()
            .set_file_name(123, "abc");
        assert_eq!(
            url.to_string(),
            "https://example.com/test?id_issue=123&page=file&deviceType=1&file=%2Fabc"
        );
    }
}
