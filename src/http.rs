#[derive(Debug, PartialEq)]
pub enum Version {
    V0_9,
    V1_0,
    V1_1,
    V2,
    V3,
}

impl Version {
    fn from(string: &str) -> Result<Version, &str> {
        use Version::*;
        match string {
            "HTTP/0.9" => Ok(V0_9),
            "HTTP/1.0" => Ok(V1_0),
            "HTTP/1.1" => Ok(V1_1),
            "HTTP/2" => Ok(V2),
            "HTTP/3" => Ok(V3),
            _ => Err("Couldn't parse version from string"),
        }
    }
}

/// HTTP methods.
#[derive(Debug, PartialEq)]
pub enum Method {
    Head,
    Get,
    Delete,
    Post,
    Patch,
    Put,
    Connect,
    Trace,
    Options,
}

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

impl Method {
    fn from(string: &str) -> Result<Method, &str> {
        use Method::*;
        match string {
            "HEAD" => Ok(Head),
            "GET" => Ok(Get),
            "DELETE" => Ok(Delete),
            "POST" => Ok(Post),
            "PATCH" => Ok(Patch),
            "PUT" => Ok(Put),
            "CONNECT" => Ok(Connect),
            "TRACE" => Ok(Trace),
            "OPTIONS" => Ok(Options),
            _ => Err("Couldn't parse method from string"),
        }
    }
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
    fn retrieve_method_from_string() {
        assert_eq!(Method::Head, Method::from("HEAD").unwrap());
        assert_eq!(Method::Get, Method::from("GET").unwrap());
        assert_eq!(Method::Delete, Method::from("DELETE").unwrap());
        assert_eq!(Method::Post, Method::from("POST").unwrap());
        assert_eq!(Method::Patch, Method::from("PATCH").unwrap());
        assert_eq!(Method::Put, Method::from("PUT").unwrap());
        assert_eq!(Method::Connect, Method::from("CONNECT").unwrap());
        assert_eq!(Method::Trace, Method::from("TRACE").unwrap());
        assert_eq!(Method::Options, Method::from("OPTIONS").unwrap());
    }

    #[test]
    #[should_panic(expected = "parse method from string")]
    fn retrieve_method_from_string_fail() {
        Method::from("GARBAGE").unwrap();
    }

    #[test]
    fn retrieve_version_from_string() {
        assert_eq!(Version::V0_9, Version::from("HTTP/0.9").unwrap());
        assert_eq!(Version::V1_0, Version::from("HTTP/1.0").unwrap());
        assert_eq!(Version::V1_1, Version::from("HTTP/1.1").unwrap());
        assert_eq!(Version::V2, Version::from("HTTP/2").unwrap());
        assert_eq!(Version::V3, Version::from("HTTP/3").unwrap());
    }

    #[test]
    #[should_panic(expected = "parse version from string")]
    fn retrieve_version_from_string_fail() {
        Version::from("HTTP/0.8").unwrap();
    }

    #[test]
    fn parse_introduction_success() {
        assert_eq!((Method::Get, String::from("index.html"), Version::V1_1), Request::parse_introduction("GET index.html HTTP/1.1").unwrap());
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
