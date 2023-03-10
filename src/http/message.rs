use super::{Body, Error, Headers, Method, Result, Status, Url, Version};
use std::io::{BufRead, Write};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum StartLine {
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

impl FromStr for StartLine {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut space_splitted_iter = s.split_ascii_whitespace();

        let method = space_splitted_iter
            .next()
            .ok_or(Error::MalformedRequestLine(
                "couldn't find the method".to_string(),
            ))?
            .parse()
            .map_err(|e| {
                Error::MalformedRequestLine(format!("couldn't parse given method: {}", e))
            })?;

        let url = space_splitted_iter
            .next()
            .ok_or(Error::MalformedRequestLine(
                "couldn't find the url".to_string(),
            ))?
            .to_string();

        let version = space_splitted_iter
            .next()
            .ok_or(Error::MalformedRequestLine(
                "couldn't find the version".to_string(),
            ))?
            .parse()
            .map_err(|e| {
                Error::MalformedRequestLine(format!("couldn't parse given version: {}", e))
            })?;

        Ok(Self::Request {
            method: method,
            url: url,
            version: version,
        })
    }
}

impl ToString for StartLine {
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
    start_line: StartLine,
    headers: Headers,
    body: Option<Body>,
}

impl Message {
    pub fn read(bufread: &mut impl BufRead) -> Result<Self> {
        // Parse header
        let mut iter = bufread
            .by_ref()
            .lines()
            .take_while(|s| s.is_ok())
            .map(|s| s.unwrap());
        let start_line = iter
            .next()
            .ok_or(Error::MalformedRequestLine(
                "couldn't find request line".to_string(),
            ))?
            .parse()?;
        let headers = Headers::read(iter)?;
        let body = headers
            .get_content_length()
            .map(|content_length| Body::read(bufread, content_length))
            .transpose()?;
        Ok(Self {
            start_line: start_line,
            headers: headers,
            body: body,
        })
    }

    pub fn write(&self, bufwrite: &mut impl Write) -> Result<()> {
        bufwrite.write(self.start_line.to_string().as_bytes())?;
        self.headers.write(bufwrite)?;
        if let Some(body) = &self.body {
            body.write(bufwrite)?;
        }
        Ok(())
    }

    pub fn new(status: Status, headers: Option<Headers>, body: Option<Body>) -> Self {
        let mut headers = headers.unwrap_or_else(Headers::new);
        body.as_ref().map(|b| headers.set_content_length(b.len()));
        Message {
            start_line: {
                StartLine::Response {
                    version: Version::V1_1,
                    status: status,
                }
            },
            headers: headers,
            body: body,
        }
    }

    pub fn start_line(&self) -> &StartLine {
        &self.start_line
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
    fn test_start_line_from_str() {
        assert_eq!(
            StartLine::Request {
                method: Method::Get,
                url: String::from("index.html"),
                version: Version::V1_1
            },
            "GET index.html HTTP/1.1".parse().unwrap()
        );
    }

    #[test]
    #[should_panic(expected = "find the version")]
    fn test_start_line_from_str_panic_if_missing_version() {
        StartLine::from_str("GET index.html").unwrap();
    }

    #[test]
    #[should_panic(expected = "find the url")]
    fn test_start_line_from_str_panic_if_missing_url() {
        StartLine::from_str("GET").unwrap();
    }

    #[test]
    #[should_panic(expected = "find the method")]
    fn test_start_line_from_str_panic_if_missing_everything() {
        StartLine::from_str("").unwrap();
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
                start_line: StartLine::Request {
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
                start_line: StartLine::Request {
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
                start_line: StartLine::Request {
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
    #[should_panic(expected = "parse given method")]
    fn test_start_line_from_str_panic_if_wrong_method() {
        StartLine::from_str("GOT / HTTP/1.1").unwrap();
    }
}
