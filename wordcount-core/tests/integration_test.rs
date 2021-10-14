use std::error;

mod common;
mod mock;

#[test]
fn test_wordcount_display() {
    const EXPECTED_DISPLAY: &str =
        "Words: 6\nLines: 3\nUnique words:\n\ttest3: 3\n\ttest2: 2\n\ttest1: 1\n";

    let word_count = common::new_wordcount();

    assert_eq!(String::from(EXPECTED_DISPLAY), format!("{}", word_count))
}

#[test]
fn test_wordcount_debug() {
    const EXPECTED_DEBUG: &str =
        "WordCount { total: 6, lines: 3, unique_words: [(\"test3\", 3), (\"test2\", 2), (\"test1\", 1)] }";

    let word_count = common::new_wordcount();

    assert_eq!(String::from(EXPECTED_DEBUG), format!("{:?}", word_count))
}

#[test]
fn test_count_words() -> Result<(), Box<dyn error::Error>> {
    const TEST: &str = "test1\ntest2 test2\ntest3 test3 test3\n";

    let config = wordcount_core::Config::new(mock::Reader::new(TEST))?;
    let word_counts = wordcount_core::count_words(config, sort_desc);

    assert_eq!(
        common::EXPECTED_TOTAL_WORDS,
        word_counts.total,
        "unexpected total words"
    );
    assert_eq!(
        common::EXPECTED_TOTAL_LINES,
        word_counts.lines,
        "unexpected total lines"
    );
    assert_eq!(
        common::EXPECTED_UNIQUE_WORDS,
        word_counts.unique_words.len(),
        "unexpected unique words"
    );

    Ok(())
}

fn sort_desc(a: u32, b: u32) -> std::cmp::Ordering {
    b.cmp(&a)
}
