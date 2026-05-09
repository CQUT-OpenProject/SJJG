fn find_substring(data: &[char], pattern: &[char]) -> Option<usize> {
    if pattern.is_empty() {
        return Some(0);
    }

    if pattern.len() > data.len() {
        return None;
    }

    for i in 0..=data.len() - pattern.len() {
        let mut same = true;

        for j in 0..pattern.len() {
            if data[i + j] != pattern[j] {
                same = false;
                break;
            }
        }

        if same {
            return Some(i);
        }
    }

    None
}

pub fn delete_substring(s1: &str, s2: &str) -> String {
    if s2.is_empty() {
        return s1.to_string();
    }

    let mut data: Vec<char> = s1.chars().collect();
    let pattern: Vec<char> = s2.chars().collect();

    while let Some(pos) = find_substring(&data, &pattern) {
        for _ in 0..pattern.len() {
            data.remove(pos);
        }
    }

    data.into_iter().collect()
}
