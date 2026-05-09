#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommonResult {
    pub max_len: usize,
    pub pos1: usize,
    pub pos2: usize,
    pub text: String,
}

pub fn max_common_substring(s: &str, t: &str) -> CommonResult {
    let a: Vec<char> = s.chars().collect();
    let b: Vec<char> = t.chars().collect();
    let mut best_len = 0;
    let mut best_pos1 = 0;
    let mut best_pos2 = 0;

    for i in 0..a.len() {
        for j in 0..b.len() {
            let mut len = 0;

            while i + len < a.len() && j + len < b.len() && a[i + len] == b[j + len] {
                len += 1;
            }

            if len > best_len {
                best_len = len;
                best_pos1 = i;
                best_pos2 = j;
            }
        }
    }

    let mut text = String::new();
    for i in best_pos1..best_pos1 + best_len {
        text.push(a[i]);
    }

    CommonResult {
        max_len: best_len,
        pos1: best_pos1,
        pos2: best_pos2,
        text,
    }
}
