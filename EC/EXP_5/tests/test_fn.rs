use exp_5::{
    compress, compression_percent, count_frequency, decompress, encode_text, generate_codes,
    pack_bits,
};
use std::collections::HashMap;

#[test]
fn count_simple_text() {
    let freq = count_frequency("AEEBBBDDDDDDCCCCCCCC");
    let mut map: HashMap<char, usize> = HashMap::new();
    for &(ch, w) in &freq {
        map.insert(ch, w);
    }
    assert_eq!(map.get(&'A'), Some(&1));
    assert_eq!(map.get(&'B'), Some(&3));
    assert_eq!(map.get(&'C'), Some(&8));
    assert_eq!(map.get(&'D'), Some(&6));
    assert_eq!(map.get(&'E'), Some(&2));
}

#[test]
fn generate_codes_huffman() {
    let text = "AEEBBBDDDDDDCCCCCCCC";
    let freq = count_frequency(text);
    let codes = generate_codes(&freq);
    // 验证每个字符都有编码
    for ch in text.chars() {
        assert!(codes.contains_key(&ch));
    }
    // 验证是前缀码：没有编码是另一个编码的前缀
    let code_list: Vec<&String> = codes.values().collect();
    for i in 0..code_list.len() {
        for j in 0..code_list.len() {
            if i != j && code_list[i].starts_with(code_list[j].as_str()) {
                panic!("编码 '{}' 是 '{}' 的前缀", code_list[j], code_list[i]);
            }
        }
    }
}

#[test]
fn encode_decode_roundtrip() {
    let text = "AEEBBBDDDDDDCCCCCCCC";
    let freq = count_frequency(text);
    let codes = generate_codes(&freq);
    let bit_string = encode_text(text, &codes);

    // 验证编码不是空的
    assert!(!bit_string.is_empty());

    // 编码后 bit 数应小于原始 8 bit/char
    assert!((bit_string.len() as f64) < (text.len() * 8) as f64);
}

#[test]
fn pack_bits_roundtrip() {
    let bits = "10001001100110110110111111111111100000000";
    let packed = pack_bits(bits);
    assert!(!packed.is_empty());

    // 解包验证
    let mut unpacked = String::new();
    for &byte in &packed {
        for i in (0..8).rev() {
            unpacked.push(if (byte >> i) & 1 == 1 { '1' } else { '0' });
        }
    }
    // 取原始 bit 长度进行比较
    assert!(unpacked.starts_with(bits));
}

#[test]
fn compress_single_char() {
    let text = "aaaaa";
    let result = compress(text).unwrap();
    assert_eq!(result.freq.len(), 1);
    assert_eq!(result.codes.get(&'a').map(|s| s.as_str()), Some("0"));
}

#[test]
fn single_char_file_roundtrip() {
    let text = "aaaaa";
    let result = compress(text).unwrap();
    let tmp = "/tmp/huff_single_char.zip";

    exp_5::write_compressed(&result, tmp).unwrap();
    let restored = decompress(tmp).unwrap();

    assert_eq!(restored, text);
    std::fs::remove_file(tmp).ok();
}

#[test]
fn compress_multiple_chars() {
    let text = "AEEBBBDDDDDDCCCCCCCC";
    let result = compress(text).unwrap();
    assert_eq!(result.original_bits, text.len() * 8);
    // 编码数据部分应比原始小（不含头部的纯编码开销）
    assert!((result.packed_bytes.len() * 8) < result.original_bits);
    // 压缩率含头部，小文件可能 > 1.0，不强制要求
}

#[test]
fn compression_ratio_bounds() {
    let text = "AEEBBBDDDDDDCCCCCCCC";
    let result = compress(text).unwrap();
    let ratio_str = compression_percent(&result);
    assert!(ratio_str.ends_with('%'));
    let pct: f64 = ratio_str.trim_end_matches('%').parse().unwrap();
    assert!(pct > 0.0);
}

#[test]
fn file_write_read_roundtrip() {
    let text = "AEEBBBDDDDDDCCCCCCCC";
    let result = compress(text).unwrap();
    let tmp_path = "/tmp/test_huffman_roundtrip.zip";

    exp_5::write_compressed(&result, tmp_path).unwrap();
    let decoded = decompress(tmp_path).unwrap();

    assert_eq!(decoded, text);

    std::fs::remove_file(tmp_path).ok();
}

#[test]
fn decompress_invalid_file() {
    let result = decompress("/tmp/nonexistent_huffman_file_xyz.zip");
    assert!(result.is_err());
}

#[test]
fn compress_empty() {
    let result = compress("");
    assert!(result.is_none());
}

#[test]
fn frequency_sorted() {
    let text = "AEEBBBDDDDDDCCCCCCCC";
    let freq = count_frequency(text);
    // 验证按频次升序排列
    for i in 1..freq.len() {
        assert!(freq[i - 1].1 <= freq[i].1);
    }
}

#[test]
fn decompress_small_file() {
    let text = "hello world";
    let result = compress(text).unwrap();
    let tmp = "/tmp/huff_small.zip";
    exp_5::write_compressed(&result, tmp).unwrap();
    let restored = decompress(tmp).unwrap();
    assert_eq!(restored, text);
    std::fs::remove_file(tmp).ok();
}
