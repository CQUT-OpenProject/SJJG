use stack_ex_4::{is_palindrome_link, is_palindrome_seq};

fn main() {
    let samples = ["Madam I am Adam", "was it a cat I saw", "Level", "abcdef"];

    for text in samples {
        println!(
            "\"{}\" -> 顺序栈: {}, 链栈: {}",
            text,
            is_palindrome_seq(text),
            is_palindrome_link(text)
        );
    }
}
