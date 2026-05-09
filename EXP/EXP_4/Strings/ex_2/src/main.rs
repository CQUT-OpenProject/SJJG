use string_ex_2::is_prefix;

fn main() {
    let prefix = "Wonder";
    let text = "Wonderful";

    println!(
        "{} 是否为 {} 的前缀: {}",
        prefix,
        text,
        is_prefix(prefix, text)
    );
}
