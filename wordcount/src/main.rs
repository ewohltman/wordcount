use std::env;
use std::error;
use std::fs;

fn main() -> Result<(), Box<dyn error::Error>> {
    let file = fs::File::open(wordcount_core::parse_file_name(env::args())?)?;
    let config = wordcount_core::Config::new(file)?;
    let word_count = wordcount_core::WordCount::new(config, |a, b| b.cmp(&a));

    print!("{}", word_count);

    Ok(())
}
