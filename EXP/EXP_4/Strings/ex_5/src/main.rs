use string_ex_5::pattern_index;

fn main() {
    let pattern = "?re";
    let text = "there are";

    match pattern_index(pattern, text) {
        Some(pos) => println!("匹配成功，起始位置: {}", pos),
        None => println!("匹配失败"),
    }
}
