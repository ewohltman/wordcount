use std::fs::File;
use std::io::Error;
use std::io::Read;

const FILE_NAME: &str = "corpus.txt";

fn read_file(file_name: &str) -> Result<String, Error> {
    let mut buffer = String::new();
    let mut f = File::open(file_name)?;

    f.read_to_string(&mut buffer)?;

    Ok(buffer)
}

fn main() -> Result<(), Error> {
    let contents = read_file(FILE_NAME)?;
    let word_count = wordcount_core::count_words(&contents)?;

    println!("{}", word_count);

    Ok(())
}
