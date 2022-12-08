pub mod method;
pub mod version;

pub use method::Method;
pub use version::Version;

use std::collections::HashMap;
pub type Headers = HashMap<String, String>;
pub type Url = String;
pub type Body = String;

pub struct Request {
    version: Version,
    method: Method,
    url: Url,
    headers: Headers,
    body: Body,
}

impl Request {
    fn parse_introduction(introduction: &str) -> Result<(Method, Url, Version), &str> {
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

    // pub fn from(head: &Vec<String>, content: &str) -> Result<Request, &str> {
    //     // Parse header
    //     let mut head_iter = head.iter();
    //     let introduction = match head_iter.next() {
    //         Some(s) => ,
    //         None => return Err("Request is malformed, could not get an initial introduction"),
    //     };

    //     Ok(())
    // }
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
}
