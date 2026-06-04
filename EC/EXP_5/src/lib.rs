use std::collections::HashMap;

/// 统计文本中各字符出现的频次
pub fn count_frequency(text: &str) -> Vec<(char, usize)> {
    let mut map: HashMap<char, usize> = HashMap::new();

    for ch in text.chars() {
        *map.entry(ch).or_insert(0) += 1;
    }

    let mut freq: Vec<(char, usize)> = map.into_iter().collect();
    freq.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
    freq
}

/// Huffman 树结点
/// 叶结点的 ch 为 Some，内部结点 ch 为 None
#[derive(Debug, Clone)]
struct HuffNode {
    ch: Option<char>,
    weight: usize,
    left: Option<usize>,
    right: Option<usize>,
}

/// 构建 Huffman 树，返回根结点在 nodes 数组中的下标
/// 约束：权值最小的结点位于左分支上
fn build_tree(freq: &[(char, usize)]) -> Option<(Vec<HuffNode>, usize)> {
    if freq.is_empty() {
        return None;
    }

    let mut nodes: Vec<HuffNode> = freq
        .iter()
        .map(|&(ch, w)| HuffNode {
            ch: Some(ch),
            weight: w,
            left: None,
            right: None,
        })
        .collect();

    if nodes.len() == 1 {
        return Some((nodes, 0));
    }

    let mut active: Vec<usize> = (0..nodes.len()).collect();

    while active.len() > 1 {
        active.sort_by_key(|&i| nodes[i].weight);

        let i1 = active[0];
        let i2 = active[1];

        active.remove(1);
        active.remove(0);

        let w = nodes[i1].weight + nodes[i2].weight;

        let (left, right) = if nodes[i1].weight <= nodes[i2].weight {
            (i1, i2)
        } else {
            (i2, i1)
        };

        let parent_idx = nodes.len();
        nodes.push(HuffNode {
            ch: None,
            weight: w,
            left: Some(left),
            right: Some(right),
        });

        active.push(parent_idx);
    }

    Some((nodes, active[0]))
}

/// 遍历 Huffman 树生成编码表
/// 左分支 = 0，右分支 = 1
pub fn generate_codes(freq: &[(char, usize)]) -> HashMap<char, String> {
    let mut codes = HashMap::new();

    if let Some((nodes, root)) = build_tree(freq) {
        let mut stack: Vec<(usize, String)> = Vec::new();
        stack.push((root, String::new()));

        while let Some((idx, code)) = stack.pop() {
            let node = &nodes[idx];

            if let Some(ch) = node.ch {
                if code.is_empty() {
                    codes.insert(ch, "0".to_string());
                } else {
                    codes.insert(ch, code);
                }
            } else {
                if let Some(left) = node.left {
                    let mut lc = code.clone();
                    lc.push('0');
                    stack.push((left, lc));
                }
                if let Some(right) = node.right {
                    let mut rc = code.clone();
                    rc.push('1');
                    stack.push((right, rc));
                }
            }
        }
    }

    codes
}

/// 将文本编码为二进制位串
pub fn encode_text(text: &str, codes: &HashMap<char, String>) -> String {
    let mut bits = String::new();

    for ch in text.chars() {
        if let Some(code) = codes.get(&ch) {
            bits.push_str(code);
        }
    }

    bits
}

/// 将位串打包为字节数组
pub fn pack_bits(bits: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut byte: u8 = 0;
    let mut count: u32 = 0;

    for bit in bits.chars() {
        byte = (byte << 1) | (bit as u8 - b'0');
        count += 1;

        if count == 8 {
            bytes.push(byte);
            byte = 0;
            count = 0;
        }
    }

    if count > 0 {
        byte <<= 8 - count;
        bytes.push(byte);
    }

    bytes
}

/// 压缩结果
pub struct CompressResult {
    pub original_bits: usize,
    pub compressed_bits: usize,
    pub compression_ratio: f64,
    pub freq: Vec<(char, usize)>,
    pub codes: HashMap<char, String>,
    pub packed_bytes: Vec<u8>,
}

/// 对文本执行 Huffman 压缩
pub fn compress(text: &str) -> Option<CompressResult> {
    if text.is_empty() {
        return None;
    }

    let freq = count_frequency(text);
    let codes = generate_codes(&freq);

    if codes.is_empty() {
        return None;
    }

    let bit_string = encode_text(text, &codes);
    let packed = pack_bits(&bit_string);

    let header_bytes = 12 + 5 * freq.len();
    let original_bits = text.len() * 8;
    let compressed_bits = (header_bytes + packed.len()) * 8;
    let compression_ratio = compressed_bits as f64 / original_bits as f64;

    Some(CompressResult {
        original_bits,
        compressed_bits,
        compression_ratio,
        freq,
        codes,
        packed_bytes: packed,
    })
}

/// 计算压缩率（返回百分比形式的字符串，如 "67.5%"）
pub fn compression_percent(result: &CompressResult) -> String {
    format!("{:.1}%", result.compression_ratio * 100.0)
}

/// 将压缩结果写入文件
/// 文件格式：
/// [4B] 魔数: "HUF5"
/// [4B] 原始字节数 (u32)
/// [4B] 字符种类数 (u32)
/// 每个字符:
///   [1B] 字符 (u8 as char)
///   [4B] 频次 (u32)
/// [剩余] 编码后的位流
pub fn write_compressed(result: &CompressResult, out_path: &str) -> std::io::Result<()> {
    use std::io::Write;

    let mut buf: Vec<u8> = Vec::new();

    buf.extend_from_slice(b"HUF5");

    let orig = (result.freq.iter().map(|&(_, w)| w).sum::<usize>()) as u32;
    buf.extend_from_slice(&orig.to_le_bytes());

    let count = result.freq.len() as u32;
    buf.extend_from_slice(&count.to_le_bytes());

    for &(ch, w) in &result.freq {
        buf.push(ch as u8);
        buf.extend_from_slice(&(w as u32).to_le_bytes());
    }

    buf.extend_from_slice(&result.packed_bytes);

    let mut file = std::fs::File::create(out_path)?;
    file.write_all(&buf)?;

    Ok(())
}

/// 解码：从压缩文件中恢复原始文本
pub fn decompress(path: &str) -> std::io::Result<String> {
    let data = std::fs::read(path)?;

    if data.len() < 12 || &data[0..4] != b"HUF5" {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "不是有效的 HUF5 压缩文件",
        ));
    }

    let orig_len = u32::from_le_bytes([data[4], data[5], data[6], data[7]]) as usize;
    let char_count = u32::from_le_bytes([data[8], data[9], data[10], data[11]]) as usize;

    let mut offset = 12;
    let mut freq: Vec<(char, usize)> = Vec::new();

    for _ in 0..char_count {
        if offset + 5 > data.len() {
            break;
        }
        let ch = data[offset] as char;
        let w = u32::from_le_bytes([
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
            data[offset + 4],
        ]) as usize;
        freq.push((ch, w));
        offset += 5;
    }

    let codes = generate_codes(&freq);

    let bit_bytes = &data[offset..];
    let mut bit_string = String::new();

    for &byte in bit_bytes {
        for i in (0..8).rev() {
            bit_string.push(if (byte >> i) & 1 == 1 { '1' } else { '0' });
        }
    }

    let mut decoded = String::with_capacity(orig_len);

    let mut current = String::new();
    let mut inverted: HashMap<String, char> = HashMap::new();
    for (&ch, code) in &codes {
        inverted.insert(code.clone(), ch);
    }

    for bit in bit_string.chars() {
        current.push(bit);

        if let Some(&ch) = inverted.get(&current) {
            decoded.push(ch);
            current.clear();

            if decoded.len() >= orig_len {
                break;
            }
        }
    }

    Ok(decoded)
}
