use std::env;
use std::error;
use std::fs;

fn main() -> Result<(), Box<dyn error::Error>> {
    let word_count = wordcount_core::count_words(
        wordcount_core::Config::new(fs::File::open(wordcount_core::parse_file_name(
            env::args(),
        )?)?)?,
        |a, b| b.cmp(&a),
    );

    print!("{}", word_count);

    Ok(())
}
