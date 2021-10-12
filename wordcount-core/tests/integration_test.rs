mod common;

#[test]
fn test_wordcount_display() {
    const EXPECTED_DISPLAY: &str =
        "Words: 6\nLines: 5\nUnique words:\n\ttest3: 3\n\ttest2: 2\n\ttest1: 1\n";

    let word_count = common::new_wordcount();

    assert_eq!(String::from(EXPECTED_DISPLAY), format!("{}", word_count))
}

#[test]
fn test_wordcount_debug() {
    const EXPECTED_DEBUG: &str =
        "WordCount { total: 6, lines: 5, unique_words: [(\"test3\", 3), (\"test2\", 2), (\"test1\", 1)] }";

    let word_count = common::new_wordcount();

    assert_eq!(String::from(EXPECTED_DEBUG), format!("{:?}", word_count))
}

#[test]
fn test_count_words() {
    const TEST: &str = "
        test1
        test2 test2
        test3 test3 test3
        ";

    let word_counts = wordcount_core::count_words(TEST, compare);

    assert_eq!(common::EXPECTED_TOTAL, word_counts.total);
    assert_eq!(common::EXPECTED_LINES, word_counts.lines);
    assert_eq!(
        common::EXPECTED_UNIQUE_WORDS,
        word_counts.unique_words.len()
    );
}

fn compare(a: u32, b: u32) -> std::cmp::Ordering {
    b.cmp(&a)
}
