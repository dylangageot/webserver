/// HTTP methods.
#[derive(EnumString, Display, Debug, PartialEq)]
pub enum Method {
    #[strum(serialize = "HEAD")]
    Head,
    #[strum(serialize = "GET")]
    Get,
    #[strum(serialize = "DELETE")]
    Delete,
    #[strum(serialize = "POST")]
    Post,
    #[strum(serialize = "PATCH")]
    Patch,
    #[strum(serialize = "PUT")]
    Put,
    #[strum(serialize = "CONNECT")]
    Connect,
    #[strum(serialize = "TRACE")]
    Trace,
    #[strum(serialize = "OPTIONS")]
    Options,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_to_string() {
        assert_eq!(Method::Head.to_string(), String::from("HEAD"));
        assert_eq!(Method::Get.to_string(), String::from("GET"));
        assert_eq!(Method::Delete.to_string(), String::from("DELETE"));
        assert_eq!(Method::Post.to_string(), String::from("POST"));
        assert_eq!(Method::Patch.to_string(), String::from("PATCH"));
        assert_eq!(Method::Put.to_string(), String::from("PUT"));
        assert_eq!(Method::Connect.to_string(), String::from("CONNECT"));
        assert_eq!(Method::Trace.to_string(), String::from("TRACE"));
        assert_eq!(Method::Options.to_string(), String::from("OPTIONS"));
    }
}
