use stack_ex_5::{brackets_match_link, brackets_match_seq};

#[test]
fn accepts_matching_braces() {
    let cases = ["fn main() { let a = { 3 + 5 }; }", "{{}}{{}}", "abc"];

    for text in cases {
        assert!(brackets_match_seq(text));
        assert!(brackets_match_link(text));
    }
}

#[test]
fn rejects_missing_or_extra_braces() {
    let cases = ["{", "}", "{{}", "{}}", "}{", "{{abc}"];

    for text in cases {
        assert!(!brackets_match_seq(text));
        assert!(!brackets_match_link(text));
    }
}
