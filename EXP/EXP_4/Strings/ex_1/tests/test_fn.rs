use string_ex_1::reverse_string;

#[test]
fn reverses_a_string() {
    assert_eq!(reverse_string("datastructure"), "erutcurtsatad");
}

#[test]
fn handles_empty_and_one_char_strings() {
    assert_eq!(reverse_string(""), "");
    assert_eq!(reverse_string("a"), "a");
}
