/// 按空白字符拆分为单词列表
pub fn tokenize(text: &str) -> Vec<&str> {
    text.split_whitespace().collect()
}

/// 合并两篇文章的单词到统一词表（按字典序排列）
pub fn build_vocabulary<'a>(tokens1: &[&'a str], tokens2: &[&'a str]) -> Vec<&'a str> {
    let mut vocab: Vec<&str> = Vec::new();

    for &w in tokens1.iter().chain(tokens2) {
        if !vocab.contains(&w) {
            vocab.push(w);
        }
    }

    vocab.sort();
    vocab
}

/// 按统一词表计算词频向量
pub fn word_frequency(tokens: &[&str], vocab: &[&str]) -> Vec<f64> {
    let mut freq = vec![0.0_f64; vocab.len()];

    for &token in tokens {
        if let Some(idx) = vocab.iter().position(|&v| v == token) {
            freq[idx] += 1.0;
        }
    }

    freq
}

/// 向量点积
pub fn dot_product(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

/// 向量模长（欧几里得范数）
pub fn magnitude(v: &[f64]) -> f64 {
    v.iter().map(|x| x * x).sum::<f64>().sqrt()
}

/// 余弦相似度，结果范围 [0, 1]
/// 若任一向量模长为 0，返回 0.0
pub fn cosine_similarity(a: &[f64], b: &[f64]) -> f64 {
    let dot = dot_product(a, b);
    let mag_a = magnitude(a);
    let mag_b = magnitude(b);

    if mag_a == 0.0 || mag_b == 0.0 {
        0.0
    } else {
        dot / (mag_a * mag_b)
    }
}
