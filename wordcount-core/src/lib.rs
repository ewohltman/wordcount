use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct WordCount {
    pub total: usize,
    pub lines: usize,
    unique_words: Vec<(String, u32)>,
}

impl fmt::Display for WordCount {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Words: {}", self.total)?;
        writeln!(f, "Lines: {}", self.lines)?;
        write!(f, "Occurrences:\n{}", self.format())
    }
}

impl WordCount {
    fn format(&self) -> String {
        let mut buffer = String::new();

        for element in &self.unique_words {
            buffer.push_str(&format!("\t{}: {}\n", element.0, element.1));
        }

        buffer
    }
}

pub fn count_words(contents: &str, compare: fn(u32, u32) -> Ordering) -> WordCount {
    let mut total: usize = 0;
    let lines = contents.lines().count();
    let mut unique_map: HashMap<&str, u32> = HashMap::new();

    for word in contents.split_whitespace() {
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
    fn test_count_words() {
        const TEST: &str = "
        test1
        test2 test2
        test3 test3 test3
        ";

        let word_counts = count_words(TEST, compare);

        assert_eq!(word_counts.total, 6);
        assert_eq!(word_counts.lines, 5);
        assert_eq!(word_counts.unique_words.len(), 3);
    }

    fn compare(a: u32, b: u32) -> std::cmp::Ordering {
        b.cmp(&a)
    }
}
