use std::env;
use std::error;
use std::fmt;

pub struct Error {
    error: String,
}

impl Error {
    pub fn new(error: &str) -> Error {
        Error {
            error: String::from(error),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

impl error::Error for Error {}

pub fn parse_file_name(mut args: env::Args) -> Result<String, Error> {
    args.next();

    let file_name = match args.next() {
        Some(file_name) => file_name,
        None => return Err(Error::new("no input file")),
    };

    Ok(file_name)
}
