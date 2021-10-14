use std::env;
use std::error;
use std::fs;

fn main() -> Result<(), Box<dyn error::Error>> {
    print!(
        "{}",
        wordcount_core::count_words(
            wordcount_core::Config::new(fs::File::open(wordcount_core::parse_file_name(
                env::args(),
            )?)?)?,
            |a, b| b.cmp(&a),
        )
    );

    Ok(())
}
