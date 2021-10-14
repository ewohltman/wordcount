use std::error;

mod common;

#[test]
fn test_arg_error_display() {
    let error = wordcount_core::Error::new(common::EXPECTED_ERROR);
    assert_eq!(String::from(common::EXPECTED_ERROR), format!("{}", error));
}

#[test]
fn test_arg_error_debug() {
    let error = wordcount_core::Error::new(common::EXPECTED_ERROR);
    assert_eq!(String::from(common::EXPECTED_ERROR), format!("{:?}", error));
}

#[test]
fn test_parse_file_name() {
    let empty_args = vec!["".to_string()].into_iter();
    let dummy_args = vec!["1".to_string(), "2".to_string()].into_iter();

    if wordcount_core::parse_file_name(empty_args).is_ok() {
        panic!("unexpected success with empty arguments");
    };

    if let Err(error) = wordcount_core::parse_file_name(dummy_args) {
        panic!("unexpected error: {}", error)
    };
}

#[test]
fn test_config_new() {
    if let Err(error) = wordcount_core::Config::new("test".as_bytes()) {
        panic!("{}", error)
    };
}

#[test]
fn test_wordcount_display() {
    const EXPECTED_DISPLAY: &str =
        "Words: 6\nLines: 3\nUnique words:\n\ttest3: 3\n\ttest2: 2\n\ttest1: 1\n";

    let word_count = common::new_wordcount();

    assert_eq!(String::from(EXPECTED_DISPLAY), format!("{}", word_count));
}

#[test]
fn test_wordcount_debug() {
    const EXPECTED_DEBUG: &str =
        "WordCount { total: 6, lines: 3, unique_words: [(\"test3\", 3), (\"test2\", 2), (\"test1\", 1)] }";

    let word_count = common::new_wordcount();

    assert_eq!(String::from(EXPECTED_DEBUG), format!("{:?}", word_count));
}

#[test]
fn test_count_words() -> Result<(), Box<dyn error::Error>> {
    const INPUT: &str = "test1\ntest2 test2\ntest3 test3 test3\n";

    let config = wordcount_core::Config::new(INPUT.as_bytes())?;
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
