use stack_ex_5::{brackets_match_link, brackets_match_seq};

fn main() {
    let samples = [
        "fn main() { let a = { 3 + 5 }; }",
        "if (a > b) { println!(\"ok\"); ",
        "}}",
    ];

    for text in samples {
        println!(
            "{}\n顺序栈检查: {}  链栈检查: {}\n",
            text,
            brackets_match_seq(text),
            brackets_match_link(text)
        );
    }
}
