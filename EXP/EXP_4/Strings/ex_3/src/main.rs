use string_ex_3::max_common_substring;

fn main() {
    let s = "student";
    let t = "deskstudy";
    let result = max_common_substring(s, t);

    println!("串 s: {}", s);
    println!("串 t: {}", t);
    println!("最长公共子串: {}", result.text);
    println!("长度: {}", result.max_len);
    println!("s 中起始位置: {}", result.pos1);
    println!("t 中起始位置: {}", result.pos2);
}
