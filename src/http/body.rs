use std::io::BufRead;
use std::io::Write;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Body(Vec<u8>);

impl Body {
    pub fn read(bufread: &mut impl BufRead, content_length: usize) -> Result<Body, &'static str> {
        let mut body: Vec<u8> = Vec::with_capacity(content_length);
        body.resize(content_length, 0);
        match bufread.read_exact(&mut body[..]) {
            Ok(_) => Ok(Body(body)),
            Err(_) => return Err("Failed reading body"),
        }
    }

    pub fn write(&self, bufwrite: &mut impl Write) -> Result<(), std::io::Error> {
        bufwrite.write_fmt(format_args!("\r\n"))?;
        bufwrite.write_all(&self.0)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl FromStr for Body {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Body(Vec::from(s)))
    }
}

impl ToString for Body {
    fn to_string(&self) -> String {
        String::from_utf8_lossy(&self.0).to_string()
    }
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::*;

    const BODY_EXAMPLE: &str = "hello world";

    fn setup_buffer_reader<'a>() -> BufReader<&'a [u8]> {
        BufReader::new(BODY_EXAMPLE.as_bytes())
    }

    #[test]
    fn test_read() {
        assert_eq!(
            Body(Vec::from(BODY_EXAMPLE.as_bytes())),
            Body::read(&mut setup_buffer_reader(), BODY_EXAMPLE.len()).unwrap()
        )
    }

    #[test]
    #[should_panic(expected = "Failed reading body")]
    fn test_read_panic_if_content_length_is_gt_than_read_from_buffer() {
        Body::read(&mut setup_buffer_reader(), BODY_EXAMPLE.len() + 1).unwrap();
    }

    #[test]
    fn test_write() {
        let mut buffer = Vec::new();
        Body(Vec::from(BODY_EXAMPLE.as_bytes()))
            .write(&mut buffer)
            .unwrap();
        assert_eq!(
            format!("\r\n{}", BODY_EXAMPLE),
            String::from_utf8_lossy(&buffer).to_string()
        );
    }
}
