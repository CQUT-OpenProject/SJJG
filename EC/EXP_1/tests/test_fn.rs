use exp_1::{cosine_similarity, build_vocabulary, word_frequency, tokenize};

/// 辅助：计算两段文本的余弦相似度
fn similarity(text1: &str, text2: &str) -> f64 {
    let tokens1 = tokenize(text1);
    let tokens2 = tokenize(text2);
    let vocab = build_vocabulary(&tokens1, &tokens2);
    let vec1 = word_frequency(&tokens1, &vocab);
    let vec2 = word_frequency(&tokens2, &vocab);
    cosine_similarity(&vec1, &vec2)
}

#[test]
fn identical_text() {
    let text = "hello world hello";
    let sim = similarity(text, text);
    assert!((sim - 1.0).abs() < 1e-9);
}

#[test]
fn completely_different() {
    let text1 = "apple banana orange";
    let text2 = "dog cat bird fish";
    let sim = similarity(text1, text2);
    assert!((sim - 0.0).abs() < 1e-9);
}

#[test]
fn partial_overlap() {
    let text1 = "apple banana orange pear";
    let text2 = "orange banana grape apple";
    // vocab: apple, banana, grape, orange, pear
    // vec1: 1, 1, 0, 1, 1
    // vec2: 1, 1, 1, 1, 0
    // dot = 1+1+0+1+0 = 3
    // |vec1| = sqrt(4) = 2, |vec2| = sqrt(4) = 2
    // cos = 3/4 = 0.75
    let sim = similarity(text1, text2);
    assert!((sim - 0.75).abs() < 1e-9);
}

#[test]
fn empty_text() {
    let sim = similarity("", "hello world");
    assert!((sim - 0.0).abs() < 1e-9);
}

#[test]
fn both_empty() {
    let sim = similarity("", "");
    assert!((sim - 0.0).abs() < 1e-9);
}

#[test]
fn known_repeated_words() {
    let text1 = "a a b b c c d";
    let text2 = "a a b b b c d d";
    // vocab: a, b, c, d
    // vec1: 2, 2, 2, 1
    // vec2: 2, 3, 1, 2
    // dot = 4+6+2+2 = 14
    // |vec1| = sqrt(4+4+4+1) = sqrt(13)
    // |vec2| = sqrt(4+9+1+4) = sqrt(18)
    // cos = 14/sqrt(234) ≈ 0.914
    let sim = similarity(text1, text2);
    let expected = 14.0 / (13.0_f64.sqrt() * 18.0_f64.sqrt());
    assert!((sim - expected).abs() < 1e-9);
}
