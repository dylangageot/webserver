use super::{Body, Headers, Method, Reason, Status, Url, Version};
use std::io::BufRead;

#[derive(Debug, PartialEq)]
pub enum Type {
    Request {
        method: Method,
        url: Url,
        version: Version,
    },
    Response {
        version: Version,
        status: Status,
        reason: Reason,
    },
}

impl Type {
    fn from(introduction: &str) -> Result<Self, &'static str> {
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
        Ok(Type::Request {
            method: method,
            url: url,
            version: version,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct Message {
    message_type: Type,
    headers: Headers,
    body: Option<Body>,
}

impl Message {
    fn parse_headers(head: impl Iterator<Item = String>) -> Option<Headers> {
        head.take_while(|s| !s.is_empty())
            .map(|s| {
                let (k, v) = s.split_at(s.find(":")?);
                Some((
                    String::from(k),
                    String::from(if v.len() > 1 { &v[2..] } else { "" }),
                ))
            })
            .collect()
    }

    pub fn from(bufread: &mut impl BufRead) -> Result<Self, &'static str> {
        // Parse header
        let mut iter = bufread.by_ref().lines().map(|s| s.unwrap());
        let message_type = match iter.next() {
            Some(s) => Type::from(&s)?,
            None => return Err("Couldn't get an initial introduction line"),
        };
        let headers = match Message::parse_headers(iter) {
            Some(h) => h,
            None => return Err("Couldn't parse headers"),
        };
        let body = match headers.get("Content-Length") {
            Some(content_length) => match content_length.parse() {
                Ok(content_length) => {
                    let mut body: Vec<u8> = Vec::with_capacity(content_length);
                    body.resize(content_length, 0);
                    match bufread.read_exact(&mut body[..]) {
                        Ok(_) => Some(String::from_utf8_lossy(&body).to_string()),
                        Err(_) => return Err("Failed reading body"),
                    }
                }
                Err(_) => return Err("Coudn't retrieve content length"),
            },
            None => None,
        };
        Ok(Self {
            message_type: message_type,
            headers: headers,
            body: body,
        })
    }

    pub fn message_type(&self) -> &Type {
        &self.message_type
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.headers.get(key)
    }

    pub fn body(&self) -> &Option<String> {
        &self.body
    }
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::*;
    #[test]
    fn parse_introduction_success() {
        assert_eq!(
            Type::Request {
                method: Method::Get,
                url: String::from("index.html"),
                version: Version::V1_1
            },
            Type::from("GET index.html HTTP/1.1").unwrap()
        );
    }

    #[test]
    #[should_panic(expected = "find the version field")]
    fn parse_introduction_version_parsing_fail() {
        Type::from("GET index.html").unwrap();
    }

    #[test]
    #[should_panic(expected = "find the url field")]
    fn parse_introduction_url_parsing_fail() {
        Type::from("GET").unwrap();
    }

    #[test]
    #[should_panic(expected = "find the method field")]
    fn parse_introduction_method_parsing_fail() {
        Type::from("").unwrap();
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
            Message {
                message_type: Type::Request {
                    method: Method::Get,
                    url: String::from("/"),
                    version: Version::V1_1
                },
                headers: Headers::from([
                    (String::from("Content-Type"), String::from("text/plain")),
                    (String::from("User-Agent"), String::from("curl"))
                ]),
                body: None
            },
            Message::from(&mut bufread).unwrap()
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
            Message {
                message_type: Type::Request {
                    method: Method::Get,
                    url: String::from("/"),
                    version: Version::V1_1
                },
                headers: Headers::from([
                    (String::from("Content-Type"), String::from("text/plain")),
                    (String::from("User-Agent"), String::from("curl"))
                ]),
                body: None
            },
            Message::from(&mut bufread).unwrap()
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
        Message::from(&mut bufread).unwrap();
    }

    #[test]
    fn from_headers_parsing_slice_index_still_success() {
        let mut bufread = BufReader::new(
            "\
GET / HTTP/1.1
Content-Type: text/plain
User-Agent:
"
            .as_bytes(),
        );
        Message::from(&mut bufread).unwrap();
    }

    #[test]
    fn from_request_success() {
        let mut bufread = BufReader::new(
            "\
GET / HTTP/1.1
Content-Type: text/plain
User-Agent: curl
Content-Length: 11

hello world"
                .as_bytes(),
        );
        assert_eq!(
            Message {
                message_type: Type::Request {
                    method: Method::Get,
                    url: String::from("/"),
                    version: Version::V1_1
                },
                headers: Headers::from([
                    (String::from("Content-Type"), String::from("text/plain")),
                    (String::from("User-Agent"), String::from("curl")),
                    (String::from("Content-Length"), String::from("11")),
                ]),
                body: Some(String::from("hello world"))
            },
            Message::from(&mut bufread).unwrap()
        );
    }

    #[test]
    #[should_panic(expected = "parse method from string")]
    fn from_introduction_line_method_parsing_fail() {
        let mut bufread = BufReader::new("GOT / HTTP/1.1".as_bytes());
        Message::from(&mut bufread).unwrap();
    }
}
