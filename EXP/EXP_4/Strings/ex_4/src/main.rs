use string_ex_4::delete_substring;

fn main() {
    let s1 = "abcxxabcxxabc";
    let s2 = "abc";

    println!("原串 s1: {}", s1);
    println!("要删除的 s2: {}", s2);
    println!("删除后: {}", delete_substring(s1, s2));
}
