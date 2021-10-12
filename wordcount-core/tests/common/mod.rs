pub const EXPECTED_TOTAL: usize = 6;
pub const EXPECTED_LINES: usize = 5;
pub const EXPECTED_UNIQUE_WORDS: usize = 3;

pub fn new_wordcount() -> wordcount_core::WordCount {
    wordcount_core::WordCount {
        total: EXPECTED_TOTAL,
        lines: EXPECTED_LINES,
        unique_words: vec![
            (String::from("test3"), 3),
            (String::from("test2"), 2),
            (String::from("test1"), 1),
        ],
    }
}
