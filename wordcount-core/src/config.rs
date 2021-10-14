use std::error;
use std::io;
use std::str;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub struct Config {
    contents: String,
}

impl Config {
    pub fn new<T: io::Read>(reader: T) -> Result<Config> {
        Ok(Config {
            contents: read(reader)?,
        })
    }

    pub fn lines(&self) -> usize {
        self.contents.lines().count()
    }

    pub fn content_iter(&self) -> str::SplitWhitespace {
        self.contents.split_whitespace()
    }
}

fn read<T: std::io::Read>(mut reader: T) -> Result<String> {
    let mut buffer = String::new();

    reader.read_to_string(&mut buffer)?;

    Ok(buffer.to_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        let output = match read("ABC".as_bytes()) {
            Ok(output) => output,
            Err(error) => panic!("unexpected error: {}", error),
        };

        assert_eq!("abc", output);
    }
}
