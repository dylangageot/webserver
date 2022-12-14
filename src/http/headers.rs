use std::collections::BTreeMap;
use std::io::Write;

#[derive(Debug, PartialEq)]
pub struct Headers(BTreeMap<String, String>);

impl Headers {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn read(head: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        match head
            .take_while(|s| !s.is_empty())
            .map(|s| {
                let (k, v) = s.split_at(s.find(":")?);
                Some((
                    String::from(k),
                    String::from(if v.len() > 1 { &v[2..] } else { "" }),
                ))
            })
            .collect()
        {
            Some(header) => Ok(Headers(header)),
            None => Err("Couldn't parse headers"),
        }
    }

    pub fn write(&self, bufwrite: &mut impl Write) -> Result<(), std::io::Error> {
        for (k, v) in &self.0 {
            bufwrite.write_fmt(format_args!("{}: {}\r\n", k, v))?;
        }
        Ok(())
    }

    pub fn get_content_length(&self) -> Option<usize> {
        self.0
            .get("Content-Length")
            .map(|size| size.parse().unwrap_or(0))
    }

    pub fn set_content_length(&mut self, size: usize) {
        self.0
            .insert(String::from("Content-Length"), size.to_string());
    }
}

impl<const N: usize> From<[(String, String); N]> for Headers {
    fn from(map: [(String, String); N]) -> Self {
        Self(BTreeMap::from_iter(map))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEADER_EXAMPLE: &str = "\
Content-Type: text/plain
Content-Length: 50
User-Agent: curl";

    fn setup_header() -> Headers {
        Headers::from([
            (String::from("Content-Type"), String::from("text/plain")),
            (String::from("Content-Length"), String::from("50")),
            (String::from("User-Agent"), String::from("curl")),
        ])
    }

    #[test]
    fn test_read() {
        assert_eq!(
            setup_header(),
            Headers::read(HEADER_EXAMPLE.lines().map(|s| s.to_string())).unwrap()
        );
    }

    #[test]
    fn test_write() {
        let headers = setup_header();
        let mut buffer = Vec::new();
        headers.write(&mut buffer).unwrap();
        assert_eq!(
            format!("Content-Length: 50\r\nContent-Type: text/plain\r\nUser-Agent: curl\r\n"),
            String::from_utf8_lossy(&buffer).to_string()
        );
    }

    #[test]
    fn test_get_content_length() {
        assert_eq!(Some(50), setup_header().get_content_length());
    }

    #[test]
    fn test_set_content_length() {
        let mut headers = setup_header();
        headers.set_content_length(40);
        assert_eq!(Some(40), headers.get_content_length());
    }

}
