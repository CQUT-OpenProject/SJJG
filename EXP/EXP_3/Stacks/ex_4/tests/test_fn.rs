use stack_ex_4::{is_palindrome_link, is_palindrome_seq};

#[test]
fn palindrome_examples_match_requirement() {
    let cases = [
        ("Madam I am Adam", false),
        ("was it a cat I saw", true),
        ("Level", true),
        ("abcdef", false),
        ("abab", false),
    ];

    for (text, expect) in cases {
        assert_eq!(is_palindrome_seq(text), expect);
        assert_eq!(is_palindrome_link(text), expect);
    }
}

#[test]
fn ignores_symbols_and_case() {
    assert!(is_palindrome_seq("Able was I, ere I saw Elba!"));
    assert!(is_palindrome_link("12321"));
    assert!(!is_palindrome_seq("123421"));
}
