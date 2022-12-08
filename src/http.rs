pub mod method;
pub mod version;

pub use method::Method;
pub use version::Version;

use std::collections::HashMap;
pub type Headers = HashMap<String, String>;
pub type Url = String;
pub type Body = String;

#[derive(Debug, PartialEq)]
pub struct Request {
    version: Version,
    method: Method,
    url: Url,
    headers: Headers,
    body: Body,
}

impl Request {
    fn parse_introduction(introduction: &str) -> Result<(Method, Url, Version), &'static str> {
        let mut space_splitted_iter = introduction.split_ascii_whitespace();
        let method = match space_splitted_iter.next() {
            Some(s) => Method::from(s)?,
            None => return Err("Couldn't find the method field in the introduction line"),
        };
        let url = match space_splitted_iter.next() {
            Some(s) => s.to_string(),
            None => return Err("Couldn't find the url field in the introduction line"),
        };
        let version = match space_splitted_iter.next() {
            Some(s) => Version::from(s)?,
            None => return Err("Couldn't find the version field in the introduction line"),
        };
        Ok((method, url, version))
    }

    fn parse_headers<'a>(head: impl Iterator<Item = &'a String>) -> Option<Headers> {
        head.map(|s| {
            let (k, v) = s.split_at(s.find(":")?);
            Some((String::from(k), String::from(&v[2..])))
        })
        .collect()
    }

    pub fn from(head: &Vec<String>, content: String) -> Result<Request, &'static str> {
        // Parse header
        let mut head_iter = head.iter();
        let (method, url, version) = match head_iter.next() {
            Some(s) => Request::parse_introduction(s)?,
            None => return Err("Couldn't get an initial introduction line"),
        };
        let headers = match Request::parse_headers(head_iter) {
            Some(h) => h,
            None => return Err("Couldn't parse headers"),
        };
        Ok(Request {
            version: version,
            method: method,
            url: url,
            headers: headers,
            body: content,
        })
    }
}

pub struct Response {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_introduction_success() {
        assert_eq!(
            (Method::Get, String::from("index.html"), Version::V1_1),
            Request::parse_introduction("GET index.html HTTP/1.1").unwrap()
        );
    }

    #[test]
    #[should_panic(expected = "find the version field")]
    fn parse_introduction_version_parsing_fail() {
        Request::parse_introduction("GET index.html").unwrap();
    }

    #[test]
    #[should_panic(expected = "find the url field")]
    fn parse_introduction_url_parsing_fail() {
        Request::parse_introduction("GET").unwrap();
    }

    #[test]
    #[should_panic(expected = "find the method field")]
    fn parse_introduction_method_parsing_fail() {
        Request::parse_introduction("").unwrap();
    }

    #[test]
    fn from_success() {
        let headers = vec![
            String::from("GET / HTTP/1.1"),
            String::from("Content-Type: text/plain"),
            String::from("User-Agent: curl"),
        ];
        assert_eq!(
            Request {
                version: Version::V1_1,
                method: Method::Get,
                url: String::from("/"),
                headers: Headers::from([
                    (String::from("Content-Type"), String::from("text/plain")),
                    (String::from("User-Agent"), String::from("curl"))
                ]),
                body: Body::from(""),
            },
            Request::from(&headers, String::from("")).unwrap()
        );
    }

    #[test]
    #[should_panic(expected = "parse headers")]
    fn from_headers_parsing_fail() {
        let headers = vec![
            String::from("GET / HTTP/1.1"),
            String::from("Content-Type: text/plain"),
            String::from("User-Agent curl"), // missing ":" here will cause an error.
        ];
        Request::from(&headers, String::from("")).unwrap();
    }

    #[test]
    #[should_panic(expected = "parse method from string")]
    fn from_introduction_line_method_parsing_fail() {
        let headers = vec![
            String::from("GOT / HTTP/1.1"),
        ];
        Request::from(&headers, String::from("")).unwrap();
    }

}
