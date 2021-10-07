use std::collections::HashMap;
use std::fs::File;
use std::io::Error;
use std::io::Read;

const FILE_NAME: &str = "corpus.txt";

fn count_words(file_name: &str) -> Result<&HashMap<&str, u32>, Error> {
    let contents = read_file(file_name)?;
    let mut word_counts: HashMap<&str, u32> = HashMap::new();

    for word in contents.split_whitespace() {
        *word_counts.entry(word).or_insert(0) += 1;
    }

    Ok(&word_counts)
}

fn read_file(file_name: &str) -> Result<String, Error> {
    let mut buffer = String::new();
    let mut f = File::open(file_name)?;

    f.read_to_string(&mut buffer)?;

    Ok(buffer)
}

fn sort(hash_map: &HashMap<&str, u32>) -> &Vec<(&str, u32)> {
    let mut vec: Vec<(&str, u32)> = hash_map.iter().map(|x| (*x.0, *x.1)).collect();

    // TODO: accept sorting function as parameter
    vec.sort_by(|a, b| b.1.cmp(&a.1));

    vec.as_ref()
}

fn format(vec: &Vec<(&str, u32)>) -> String {
    let mut buffer = String::new();

    for element in vec {
        buffer.push_str(&format!("{}: {}\n", element.0, element.1));
    }

    buffer
}

fn main() {
    let mut word_counts = match count_words(FILE_NAME) {
        Ok(word_counts) => word_counts,
        Err(err) => {
            println!("Error counting words in file {}: {}", FILE_NAME, err);
            return;
        }
    };

    println!("Total words: {}", word_counts.len());
    println!("Word counts:\n{}", format(sort(word_counts)));
}
