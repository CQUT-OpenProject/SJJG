pub fn is_prefix(prefix: &str, text: &str) -> bool {
    let p: Vec<char> = prefix.chars().collect();
    let t: Vec<char> = text.chars().collect();

    if p.len() > t.len() {
        return false;
    }

    for i in 0..p.len() {
        if p[i] != t[i] {
            return false;
        }
    }

    true
}
