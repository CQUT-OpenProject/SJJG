use std::fs;
use std::process;

use exp_5::{compress, compression_percent, write_compressed};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("用法: {} <输入文件路径> [输出文件路径]", args[0]);
        eprintln!("  默认输出文件: huffmandata.zip");
        process::exit(1);
    }

    let input_path = &args[1];
    let output_path = if args.len() >= 3 {
        args[2].clone()
    } else {
        "huffmandata.zip".to_string()
    };

    let text = fs::read_to_string(input_path).unwrap_or_else(|err| {
        eprintln!("无法读取输入文件 {}: {}", input_path, err);
        process::exit(1);
    });

    if text.is_empty() {
        eprintln!("输入文件为空，无需压缩");
        process::exit(0);
    }

    let result = compress(&text).expect("压缩失败");

    println!("=== Huffman 编码压缩结果 ===");
    println!("原始文件: {}", input_path);
    println!("输出文件: {}", output_path);
    println!(
        "原始大小: {} 字节 ({} bit)",
        text.len(),
        result.original_bits
    );
    let total_bytes = result.compressed_bits / 8;
    println!(
        "压缩后大小: {} 字节 ({} bit)",
        total_bytes, result.compressed_bits
    );
    println!("压缩率: {} (压缩后/原始)", compression_percent(&result));
    println!();
    println!("=== 字符编码表 ===");

    let mut sorted_chars: Vec<(char, usize)> = result.freq.clone();
    sorted_chars.sort_by(|a, b| a.1.cmp(&b.1));
    let codes = &result.codes;

    for (ch, freq) in &sorted_chars {
        if let Some(code) = codes.get(ch) {
            println!("  '{}' (频次={}): {}", ch, freq, code);
        }
    }

    if let Err(err) = write_compressed(&result, &output_path) {
        eprintln!("写入压缩文件失败: {}", err);
        process::exit(1);
    }

    println!();
    println!("压缩文件已保存到: {}", output_path);
}
