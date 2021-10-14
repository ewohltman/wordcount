pub mod args;
pub mod config;

pub use crate::args::*;
pub use crate::config::Config;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct WordCount {
    pub total: usize,
    pub lines: usize,
    pub unique_words: Vec<(String, u32)>,
}

impl fmt::Display for WordCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Words: {}", self.total)?;
        writeln!(f, "Lines: {}", self.lines)?;
        write!(f, "Unique words:\n{}", self.format_unique_words())
    }
}

impl WordCount {
    fn format_unique_words(&self) -> String {
        let mut buffer = String::new();

        for element in &self.unique_words {
            buffer.push_str(&format!("\t{}: {}\n", element.0, element.1));
        }

        buffer
    }
}

pub fn count_words(config: Config, compare: fn(u32, u32) -> Ordering) -> WordCount {
    let mut total: usize = 0;
    let lines = config.contents.lines().count();
    let mut unique_map: HashMap<&str, u32> = HashMap::new();

    for word in config.contents.split_whitespace() {
        total += 1;
        *unique_map.entry(word).or_insert(0) += 1;
    }

    WordCount {
        lines,
        total,
        unique_words: sort(unique_map, compare),
    }
}

fn sort(hash_map: HashMap<&str, u32>, compare: fn(u32, u32) -> Ordering) -> Vec<(String, u32)> {
    let mut vec: Vec<(String, u32)> = hash_map
        .iter()
        .map(|x| (String::from(*x.0), *x.1))
        .collect();

    vec.sort_by(|a, b| compare(a.1, b.1));

    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wordcount_format_unique_words() {
        const EXPECTED_FORMAT: &str = "\ttest3: 3\n\ttest2: 2\n\ttest1: 1\n";

        let word_count = WordCount {
            total: 0,
            lines: 0,
            unique_words: vec![
                (String::from("test3"), 3),
                (String::from("test2"), 2),
                (String::from("test1"), 1),
            ],
        };

        assert_eq!(
            String::from(EXPECTED_FORMAT),
            word_count.format_unique_words()
        )
    }

    #[test]
    fn test_sort() {
        const TEST_KEY_1: &str = "test1";
        const TEST_VALUE_1: u32 = 1;

        const TEST_KEY_2: &str = "test2";
        const TEST_VALUE_2: u32 = 2;

        const TEST_KEY_3: &str = "test3";
        const TEST_VALUE_3: u32 = 3;

        let expected_vec = vec![
            (String::from(TEST_KEY_3), TEST_VALUE_3),
            (String::from(TEST_KEY_2), TEST_VALUE_2),
            (String::from(TEST_KEY_1), TEST_VALUE_1),
        ];

        let mut hash_map: HashMap<&str, u32> = HashMap::new();
        hash_map.insert(TEST_KEY_1, TEST_VALUE_1);
        hash_map.insert(TEST_KEY_2, TEST_VALUE_2);
        hash_map.insert(TEST_KEY_3, TEST_VALUE_3);

        assert_eq!(expected_vec, sort(hash_map, compare))
    }

    fn compare(a: u32, b: u32) -> std::cmp::Ordering {
        b.cmp(&a)
    }
}
