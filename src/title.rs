use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

pub fn find_latest_issue(content: String) -> Option<u32> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"\.\./\.\./[^,]+,([0-9]+),"#).unwrap();
    }

    if let Some(caps) = RE.captures_iter(&content).next() {
        u32::from_str(caps.get(1)?.as_str()).ok()
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_number() {
        let text = r#"this is test
and a link <a href="../../test,123,thing"> this is ok
test test
test"#
            .into();
        assert_eq!(Some(123), find_latest_issue(text));
    }

    #[test]
    fn does_not_find_the_number() {
        let text = r#"this is test
and a link <a href="../test,123,thing"> this is ok
test test
test"#
            .into();
        assert_eq!(None, find_latest_issue(text));
    }

    #[test]
    fn does_not_find_the_number_2() {
        let text = r#"this is test
and a link <a href="12/34/test,123,thing"> this is ok
test test
test"#
            .into();
        assert_eq!(None, find_latest_issue(text));
    }
}
