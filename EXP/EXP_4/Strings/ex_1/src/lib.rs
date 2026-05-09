pub fn reverse_string(text: &str) -> String {
    let mut chars: Vec<char> = text.chars().collect();

    if chars.len() < 2 {
        return text.to_string();
    }

    let mut i = 0;
    let mut j = chars.len() - 1;

    while i < j {
        chars.swap(i, j);
        i += 1;
        j -= 1;
    }

    chars.into_iter().collect()
}
