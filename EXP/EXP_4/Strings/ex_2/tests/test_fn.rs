use string_ex_2::is_prefix;

#[test]
fn judges_prefix_strings() {
    assert!(is_prefix("Wonder", "Wonderful"));
    assert!(is_prefix("", "abc"));
    assert!(!is_prefix("Word", "Wonderful"));
    assert!(!is_prefix("Wonderful", "Wonder"));
}
