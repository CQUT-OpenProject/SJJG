use string_ex_5::pattern_index;

#[test]
fn finds_pattern_with_question_mark() {
    assert_eq!(pattern_index("?re", "there are"), Some(2));
    assert_eq!(pattern_index("t?e", "there are"), Some(0));
}

#[test]
fn rejects_non_matching_pattern() {
    assert_eq!(pattern_index("a?z", "there are"), None);
    assert_eq!(pattern_index("too long", "short"), None);
}

#[test]
fn empty_pattern_matches_start() {
    assert_eq!(pattern_index("", "abc"), Some(0));
}
