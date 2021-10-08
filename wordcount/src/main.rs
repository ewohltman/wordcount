use std::env;
use std::error;
use std::fmt;
use std::fs;
use std::io::Read;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

struct ArgError {
    error: String,
}

impl fmt::Display for ArgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl fmt::Debug for ArgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

impl error::Error for ArgError {}

fn read_file(file_name: &str) -> Result<String> {
    let mut buffer = String::new();
    let mut f = fs::File::open(file_name)?;

    f.read_to_string(&mut buffer)?;

    Ok(buffer)
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(Box::new(ArgError {
            error: String::from("no input file"),
        }));
    }

    let file_name = &args[1];
    let contents = read_file(file_name)?;
    let word_count = wordcount_core::count_words(&contents, |a, b| b.cmp(&a))?;

    print!("{}", word_count);

    Ok(())
}
