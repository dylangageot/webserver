use std::collections::HashMap;
use std::io::Write;

#[derive(Debug, PartialEq)]
pub struct Headers(HashMap<String, String>);

impl Headers {
    pub fn new() -> Self {
        Self(HashMap::new())
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
        Self(HashMap::from_iter(map))
    }
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::*;

    fn setup_headers() -> String {
        String::from(
            "\
Content-Type: text/plain
User-Agent: curl",
        )
    }

    #[test]
    fn test_read() {
        let header_lines = setup_headers();
        assert_eq!(
            Headers::from([
                (String::from("Content-Type"), String::from("text/plain")),
                (String::from("User-Agent"), String::from("curl"))
            ]),
            Headers::read(header_lines.lines().map(|s| s.to_string())).unwrap()
        );
    }
}
