pub const EXPECTED_TOTAL_WORDS: usize = 6;
pub const EXPECTED_TOTAL_LINES: usize = 3;
pub const EXPECTED_UNIQUE_WORDS: usize = 3;

pub fn new_wordcount() -> wordcount_core::WordCount {
    wordcount_core::WordCount {
        total: EXPECTED_TOTAL_WORDS,
        lines: EXPECTED_TOTAL_LINES,
        unique_words: vec![
            (String::from("test3"), 3),
            (String::from("test2"), 2),
            (String::from("test1"), 1),
        ],
    }
}
