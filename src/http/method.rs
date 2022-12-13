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

impl Method {
    pub fn from(string: &str) -> Result<Method, &'static str> {
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

use std::fmt;
impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Head => String::from("HEAD"),
            Self::Get => String::from("GET"),
            Self::Delete => String::from("DELETE"),
            Self::Post => String::from("POST"),
            Self::Patch => String::from("PATCH"),
            Self::Put => String::from("PUT"),
            Self::Connect => String::from("CONNECT"),
            Self::Trace => String::from("TRACE"),
            Self::Options => String::from("OPTIONS"),
        })
    }
}

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
}
