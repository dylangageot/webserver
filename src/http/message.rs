use super::{Body, Headers, Method, Status, Url, Version};
use std::io::{BufRead, Write};
use std::str::FromStr;

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
    },
}

impl FromStr for Type {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut space_splitted_iter = s.split_ascii_whitespace();
        let method = match space_splitted_iter.next() {
            Some(s) => match Method::from_str(s) {
                Ok(method) => method,
                Err(_) => return Err("Couldn't parse method from string in the introduction line"),
            },
            None => return Err("Couldn't find the method field in the introduction line"),
        };
        let url = match space_splitted_iter.next() {
            Some(s) => s.to_string(),
            None => return Err("Couldn't find the url field in the introduction line"),
        };
        let version = match space_splitted_iter.next() {
            Some(s) => match Version::from_str(s) {
                Ok(version) => version,
                Err(_) => {
                    return Err("Couldn't parse version from string in the introduction line")
                }
            },
            None => return Err("Couldn't find the version field in the introduction line"),
        };
        Ok(Type::Request {
            method: method,
            url: url,
            version: version,
        })
    }
}

impl ToString for Type {
    fn to_string(&self) -> String {
        match self {
            Self::Request {
                method,
                url,
                version,
            } => format!("{} {} {}\r\n", method, url, version),
            Self::Response { version, status } => {
                format!("{} {}\r\n", version, status)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Message {
    message_type: Type,
    headers: Headers,
    body: Option<Body>,
}

impl Message {
    pub fn read(bufread: &mut impl BufRead) -> Result<Self, &'static str> {
        // Parse header
        let mut iter = bufread.by_ref().lines().map(|s| s.unwrap());
        let message_type = match iter.next() {
            Some(s) => Type::from_str(&s)?,
            None => return Err("Couldn't get an initial introduction line"),
        };
        let headers = Headers::read(iter)?;
        let body = match headers.get_content_length() {
            Some(content_length) => Some(Body::read(bufread, content_length)?),
            None => None,
        };
        Ok(Self {
            message_type: message_type,
            headers: headers,
            body: body,
        })
    }

    pub fn write(&self, bufwrite: &mut impl Write) -> Result<(), std::io::Error> {
        bufwrite.write(self.message_type.to_string().as_bytes())?;
        self.headers.write(bufwrite)?;
        if let Some(body) = &self.body {
            body.write(bufwrite)?;
        }
        Ok(())
    }

    pub fn new(status: Status, headers: Option<Headers>, body: Option<Body>) -> Self {
        let mut headers = headers.unwrap_or_else(Headers::new);
        match &body {
            Some(body) => {
                headers.set_content_length(body.len());
            }
            _ => (),
        };
        Message {
            message_type: {
                Type::Response {
                    version: Version::V1_1,
                    status: status,
                }
            },
            headers: headers,
            body: body,
        }
    }

    pub fn message_type(&self) -> &Type {
        &self.message_type
    }

    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    pub fn body(&self) -> &Option<Body> {
        &self.body
    }
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::*;
    #[test]
    fn test_type_from_str() {
        assert_eq!(
            Type::Request {
                method: Method::Get,
                url: String::from("index.html"),
                version: Version::V1_1
            },
            Type::from_str("GET index.html HTTP/1.1").unwrap()
        );
    }

    #[test]
    #[should_panic(expected = "find the version field")]
    fn test_type_from_str_panic_if_missing_version() {
        Type::from_str("GET index.html").unwrap();
    }

    #[test]
    #[should_panic(expected = "find the url field")]
    fn test_type_from_str_panic_if_missing_url() {
        Type::from_str("GET").unwrap();
    }

    #[test]
    #[should_panic(expected = "find the method field")]
    fn test_type_from_str_panic_if_missing_everything() {
        Type::from_str("").unwrap();
    }

    #[test]
    fn test_message_read_request_without_body() {
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
            Message::read(&mut bufread).unwrap()
        );
    }

    #[test]
    fn test_message_read_do_not_parse_beyond_headers_if_no_content_length_defined() {
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
            Message::read(&mut bufread).unwrap()
        );
    }

    #[test]
    #[should_panic(expected = "parse headers")]
    fn test_message_read_headers_parse_fail_malformed() {
        let mut bufread = BufReader::new(
            "\
GET / HTTP/1.1
Content-Type: text/plain
User-Agent: curl
Should failed since it misses a colon
"
            .as_bytes(),
        );
        Message::read(&mut bufread).unwrap();
    }

    #[test]
    fn test_message_read_headers_parse_fail_missing_value() {
        let mut bufread = BufReader::new(
            "\
GET / HTTP/1.1
Content-Type: text/plain
User-Agent:
"
            .as_bytes(),
        );
        Message::read(&mut bufread).unwrap();
    }

    #[test]
    fn test_message_read_request_with_body() {
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
                body: Some(Body::from_str("hello world").unwrap())
            },
            Message::read(&mut bufread).unwrap()
        );
    }

    #[test]
    #[should_panic(expected = "parse method from string")]
    fn test_type_from_str_panic_if_wrong_method() {
        Type::from_str("GOT / HTTP/1.1").unwrap();
    }
}
