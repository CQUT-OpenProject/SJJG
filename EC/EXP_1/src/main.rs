use std::env;
use std::fs;
use std::process;

use exp_1::{cosine_similarity, build_vocabulary, tokenize, word_frequency};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("用法: {} <文章1路径> <文章2路径>", args[0]);
        process::exit(1);
    }

    let text1 = fs::read_to_string(&args[1]).unwrap_or_else(|err| {
        eprintln!("无法读取文件 {}: {}", args[1], err);
        process::exit(1);
    });

    let text2 = fs::read_to_string(&args[2]).unwrap_or_else(|err| {
        eprintln!("无法读取文件 {}: {}", args[2], err);
        process::exit(1);
    });

    let tokens1 = tokenize(&text1);
    let tokens2 = tokenize(&text2);

    println!("文章1 单词数: {}", tokens1.len());
    println!("文章2 单词数: {}", tokens2.len());

    // 合并两篇文章去重后的词表
    let vocab = build_vocabulary(&tokens1, &tokens2);
    println!("合并词表大小: {}", vocab.len());

    // 计算词频向量
    let vec1 = word_frequency(&tokens1, &vocab);
    let vec2 = word_frequency(&tokens2, &vocab);

    // 余弦相似度
    let similarity = cosine_similarity(&vec1, &vec2);

    println!("两篇文章余弦相似度: {:.6}", similarity);
}
