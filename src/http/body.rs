use super::{Error, Result};
use std::io::BufRead;
use std::io::Write;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Body(Vec<u8>);

impl Body {
    pub fn read(bufread: &mut impl BufRead, content_length: usize) -> Result<Self> {
        let mut body: Vec<u8> = Vec::with_capacity(content_length);
        body.resize(content_length, 0);
        bufread
            .read_exact(&mut body[..])
            .map_err(|e| Error::Io(e))?;
        Ok(Body(body))
    }

    pub fn write(&self, bufwrite: &mut impl Write) -> Result<()> {
        bufwrite.write_fmt(format_args!("\r\n"))?;
        bufwrite.write_all(&self.0)?;
        Ok(())
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

impl From<Vec<u8>> for Body {
    fn from(vec: Vec<u8>) -> Self {
        Body(vec)
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

    fn setup_body() -> Body {
        Body(Vec::from(BODY_EXAMPLE.as_bytes()))
    }

    #[test]
    fn test_read() {
        assert_eq!(
            setup_body(),
            Body::read(&mut setup_buffer_reader(), BODY_EXAMPLE.len()).unwrap()
        )
    }

    #[test]
    #[should_panic(expected = "failed to fill whole buffer")]
    fn test_read_panic_if_content_length_is_gt_than_read_from_buffer() {
        Body::read(&mut setup_buffer_reader(), BODY_EXAMPLE.len() + 1).unwrap();
    }

    #[test]
    fn test_write() {
        let mut buffer = Vec::new();
        setup_body().write(&mut buffer).unwrap();
        assert_eq!(
            format!("\r\n{}", BODY_EXAMPLE),
            String::from_utf8_lossy(&buffer).to_string()
        );
    }

    #[test]
    fn test_len() {
        assert_eq!(11, setup_body().len())
    }

    #[test]
    fn test_from_str() {
        assert_eq!(setup_body(), Body::from_str(BODY_EXAMPLE).unwrap())
    }

    #[test]
    fn test_to_string() {
        assert_eq!(BODY_EXAMPLE, setup_body().to_string())
    }
}
