use std::error;
use std::io;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub struct Config {
    pub(crate) contents: String,
}

impl Config {
    pub fn new<T: io::Read>(reader: T) -> Result<Config> {
        let contents = read(reader)?;

        Ok(Config { contents })
    }
}

fn read<T: std::io::Read>(mut reader: T) -> Result<String> {
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;

    Ok(buffer)
}
