#[derive(EnumString, Display, Debug, PartialEq)]
pub enum Version {
    #[strum(serialize = "HTTP/0.9")]
    V0_9,
    #[strum(serialize = "HTTP/1.0")]
    V1_0,
    #[strum(serialize = "HTTP/1.1")]
    V1_1,
    #[strum(serialize = "HTTP/2")]
    V2,
    #[strum(serialize = "HTTP/3")]
    V3,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_to_string() {
        assert_eq!(Version::V0_9.to_string(), String::from("HTTP/0.9"));
        assert_eq!(Version::V1_0.to_string(), String::from("HTTP/1.0"));
        assert_eq!(Version::V1_1.to_string(), String::from("HTTP/1.1"));
        assert_eq!(Version::V2.to_string(), String::from("HTTP/2"));
        assert_eq!(Version::V3.to_string(), String::from("HTTP/3"));
    }
}
