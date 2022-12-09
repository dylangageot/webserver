pub mod method;
pub mod version;

pub use method::Method;
pub use version::Version;

use std::{collections::HashMap, io::BufRead};
pub type Headers = HashMap<String, String>;
pub type Url = String;
pub type Body = String;

#[derive(Debug, PartialEq)]
pub struct Header {
    version: Version,
    method: Method,
    url: Url,
    headers: Headers,
}

impl Header {
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

    fn parse_headers(head: impl Iterator<Item = String>) -> Option<Headers> {
        head.take_while(|s| !s.is_empty())
            .map(|s| {
                let (k, v) = s.split_at(s.find(":")?);
                Some((String::from(k), String::from(&v[2..])))
            })
            .collect()
    }

    pub fn from(bufread: &mut impl BufRead) -> Result<Self, &'static str> {
        let mut iter = bufread.by_ref().lines().map(|s| s.unwrap());
        // Parse header
        let (method, url, version) = match iter.next() {
            Some(s) => Header::parse_introduction(&s)?,
            None => return Err("Couldn't get an initial introduction line"),
        };
        let headers = match Header::parse_headers(iter) {
            Some(h) => h,
            None => return Err("Couldn't parse headers"),
        };
        Ok(Self {
            method: method,
            url: url,
            version: version,
            headers: headers,
        })
    }

    pub fn content_length(&self) -> Result<Option<usize>, &'static str> {
        match self.headers.get("Content-Length") {
            Some(content_length) => match content_length.parse() {
                Ok(length) => Ok(Some(length)),
                Err(_) => Err("Coudn't retrieve content length"),
            },
            None => Ok(None),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Request {
    header: Header,
    body: Option<Body>,
}

impl Request {
    pub fn from(bufread: &mut impl BufRead) -> Result<Self, &'static str> {
        let header = Header::from(bufread)?;
        let body = match header.content_length()? {
            Some(content_length) => {
                let mut body: Vec<u8> = Vec::with_capacity(content_length);
                unsafe {
                    body.set_len(content_length);
                }
                match bufread.read_exact(&mut body[..]) {
                    Ok(_) => Some(String::from_utf8_lossy(&body).to_string()),
                    Err(_) => return Err("Failed reading body"),
                }
            }
            None => None,
        };
        Ok(Self {
            header: header,
            body: body,
        })
    }
}

pub struct Response {}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::*;
    #[test]
    fn parse_introduction_success() {
        assert_eq!(
            (Method::Get, String::from("index.html"), Version::V1_1),
            Header::parse_introduction("GET index.html HTTP/1.1").unwrap()
        );
    }

    #[test]
    #[should_panic(expected = "find the version field")]
    fn parse_introduction_version_parsing_fail() {
        Header::parse_introduction("GET index.html").unwrap();
    }

    #[test]
    #[should_panic(expected = "find the url field")]
    fn parse_introduction_url_parsing_fail() {
        Header::parse_introduction("GET").unwrap();
    }

    #[test]
    #[should_panic(expected = "find the method field")]
    fn parse_introduction_method_parsing_fail() {
        Header::parse_introduction("").unwrap();
    }

    #[test]
    fn from_success() {
        let mut bufread = BufReader::new(
            "\
GET / HTTP/1.1
Content-Type: text/plain
User-Agent: curl
"
            .as_bytes(),
        );
        assert_eq!(
            Header {
                version: Version::V1_1,
                method: Method::Get,
                url: String::from("/"),
                headers: Headers::from([
                    (String::from("Content-Type"), String::from("text/plain")),
                    (String::from("User-Agent"), String::from("curl"))
                ])
            },
            Header::from(&mut bufread).unwrap()
        );
    }

    #[test]
    fn from_do_not_parse_beyond_empty_line() {
        let mut bufread = BufReader::new(
            "\
GET / HTTP/1.1
Content-Type: text/plain
User-Agent: curl

Test: Beyond empty line
"
            .as_bytes(),
        );
        assert_eq!(
            Header {
                version: Version::V1_1,
                method: Method::Get,
                url: String::from("/"),
                headers: Headers::from([
                    (String::from("Content-Type"), String::from("text/plain")),
                    (String::from("User-Agent"), String::from("curl"))
                ])
            },
            Header::from(&mut bufread).unwrap()
        );
    }

    #[test]
    #[should_panic(expected = "parse headers")]
    fn from_headers_parsing_fail() {
        let mut bufread = BufReader::new(
            "\
GET / HTTP/1.1
Content-Type: text/plain
User-Agent: curl
Should failed since it misses a colon
"
            .as_bytes(),
        );
        Header::from(&mut bufread).unwrap();
    }

    #[test]
    #[should_panic(expected = "parse method from string")]
    fn from_introduction_line_method_parsing_fail() {
        let mut bufread = BufReader::new("GOT / HTTP/1.1".as_bytes());
        Request::from(&mut bufread).unwrap();
    }
}
