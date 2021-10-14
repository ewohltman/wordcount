use std::error;
use std::io;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub struct Config {
    pub contents: String,
}

impl Config {
    pub fn new<T: io::Read>(reader: T) -> Result<Config> {
        Ok(Config {
            contents: read(reader)?,
        })
    }
}

fn read<T: std::io::Read>(mut reader: T) -> Result<String> {
    let mut buffer = String::new();

    reader.read_to_string(&mut buffer)?;

    Ok(buffer.to_lowercase())
}
