fn pattern_matches(pattern: &[char], data: &[char], start: usize) -> bool {
    for i in 0..pattern.len() {
        if pattern[i] != '?' && pattern[i] != data[start + i] {
            return false;
        }
    }

    true
}

pub fn pattern_index(pattern: &str, text: &str) -> Option<usize> {
    let p: Vec<char> = pattern.chars().collect();
    let t: Vec<char> = text.chars().collect();

    if p.is_empty() {
        return Some(0);
    }

    if p.len() > t.len() {
        return None;
    }

    for i in 0..=t.len() - p.len() {
        if pattern_matches(&p, &t, i) {
            return Some(i);
        }
    }

    None
}
