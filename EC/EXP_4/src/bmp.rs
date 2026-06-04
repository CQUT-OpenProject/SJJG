use std::fs;
use std::io;
use std::path::Path;

/// BMP 文件头，共 14 字节
#[derive(Clone, Debug)]
pub struct FileHeader {
    pub bf_type: [u8; 2],
    pub bf_size: u32,
    pub bf_reserved1: u16,
    pub bf_reserved2: u16,
    pub bf_off_bits: u32,
}

/// BMP 信息头，共 40 字节（BITMAPINFOHEADER）
#[derive(Clone, Debug)]
pub struct InfoHeader {
    pub bi_size: u32,
    pub bi_width: i32,
    pub bi_height: i32,
    pub bi_planes: u16,
    pub bi_bit_count: u16,
    pub bi_compression: u32,
    pub bi_size_image: u32,
    pub bi_x_pels_per_meter: i32,
    pub bi_y_pels_per_meter: i32,
    pub bi_clr_used: u32,
    pub bi_clr_important: u32,
}

/// RGBQUAD 调色板项
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RgbQuad {
    pub blue: u8,
    pub green: u8,
    pub red: u8,
    pub reserved: u8,
}

/// 每行像素占用的字节数（4 字节对齐）
pub fn line_bytes(width: u32, bit_count: u16) -> u32 {
    ((width * bit_count as u32 + 31) / 32) * 4
}

fn read_u16_le(buf: &[u8], off: usize) -> u16 {
    u16::from_le_bytes([buf[off], buf[off + 1]])
}

fn read_u32_le(buf: &[u8], off: usize) -> u32 {
    u32::from_le_bytes([buf[off], buf[off + 1], buf[off + 2], buf[off + 3]])
}

fn read_i32_le(buf: &[u8], off: usize) -> i32 {
    i32::from_le_bytes([buf[off], buf[off + 1], buf[off + 2], buf[off + 3]])
}

fn write_u16_le(buf: &mut [u8], off: usize, v: u16) {
    let b = v.to_le_bytes();
    buf[off] = b[0];
    buf[off + 1] = b[1];
}

fn write_u32_le(buf: &mut [u8], off: usize, v: u32) {
    let b = v.to_le_bytes();
    buf[off] = b[0];
    buf[off + 1] = b[1];
    buf[off + 2] = b[2];
    buf[off + 3] = b[3];
}

fn write_i32_le(buf: &mut [u8], off: usize, v: i32) {
    let b = v.to_le_bytes();
    buf[off] = b[0];
    buf[off + 1] = b[1];
    buf[off + 2] = b[2];
    buf[off + 3] = b[3];
}

/// 读取 BMP 文件头
pub fn read_file_header(buf: &[u8]) -> io::Result<FileHeader> {
    if buf.len() < 14 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "BMP 文件头过短"));
    }
    Ok(FileHeader {
        bf_type: [buf[0], buf[1]],
        bf_size: read_u32_le(buf, 2),
        bf_reserved1: read_u16_le(buf, 6),
        bf_reserved2: read_u16_le(buf, 8),
        bf_off_bits: read_u32_le(buf, 10),
    })
}

/// 读取 BMP 信息头
pub fn read_info_header(buf: &[u8], off: usize) -> io::Result<InfoHeader> {
    if buf.len() < off + 40 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "BMP 信息头过短"));
    }
    Ok(InfoHeader {
        bi_size: read_u32_le(buf, off),
        bi_width: read_i32_le(buf, off + 4),
        bi_height: read_i32_le(buf, off + 8),
        bi_planes: read_u16_le(buf, off + 12),
        bi_bit_count: read_u16_le(buf, off + 14),
        bi_compression: read_u32_le(buf, off + 16),
        bi_size_image: read_u32_le(buf, off + 20),
        bi_x_pels_per_meter: read_i32_le(buf, off + 24),
        bi_y_pels_per_meter: read_i32_le(buf, off + 28),
        bi_clr_used: read_u32_le(buf, off + 32),
        bi_clr_important: read_u32_le(buf, off + 36),
    })
}

/// 将信息头写入缓冲区
pub fn write_info_header(buf: &mut [u8], off: usize, h: &InfoHeader) {
    write_u32_le(buf, off, h.bi_size);
    write_i32_le(buf, off + 4, h.bi_width);
    write_i32_le(buf, off + 8, h.bi_height);
    write_u16_le(buf, off + 12, h.bi_planes);
    write_u16_le(buf, off + 14, h.bi_bit_count);
    write_u32_le(buf, off + 16, h.bi_compression);
    write_u32_le(buf, off + 20, h.bi_size_image);
    write_i32_le(buf, off + 24, h.bi_x_pels_per_meter);
    write_i32_le(buf, off + 28, h.bi_y_pels_per_meter);
    write_u32_le(buf, off + 32, h.bi_clr_used);
    write_u32_le(buf, off + 36, h.bi_clr_important);
}

/// 将文件头写入缓冲区
pub fn write_file_header(buf: &mut [u8], h: &FileHeader) {
    buf[0] = h.bf_type[0];
    buf[1] = h.bf_type[1];
    write_u32_le(buf, 2, h.bf_size);
    write_u16_le(buf, 6, h.bf_reserved1);
    write_u16_le(buf, 8, h.bf_reserved2);
    write_u32_le(buf, 10, h.bf_off_bits);
}

/// 从整个文件字节中切片出调色板
pub fn read_palette(buf: &[u8], off: usize, n: usize) -> Vec<RgbQuad> {
    let mut p = Vec::with_capacity(n);
    for i in 0..n {
        let b = off + i * 4;
        p.push(RgbQuad {
            blue: buf[b],
            green: buf[b + 1],
            red: buf[b + 2],
            reserved: buf[b + 3],
        });
    }
    p
}

/// 将调色板写入缓冲区
pub fn write_palette(buf: &mut [u8], off: usize, pal: &[RgbQuad]) {
    for (i, q) in pal.iter().enumerate() {
        let b = off + i * 4;
        buf[b] = q.blue;
        buf[b + 1] = q.green;
        buf[b + 2] = q.red;
        buf[b + 3] = q.reserved;
    }
}

/// 读取像素区为 2D 矩阵（行主序，行号 0 对应图像顶行）
/// top_down 为 true 时，说明 BMP 的 biHeight 为负，文件中第一行就是图像顶行
pub fn read_pixels(
    buf: &[u8],
    off: usize,
    width: u32,
    height: u32,
    bit_count: u16,
    top_down: bool,
) -> Vec<Vec<u8>> {
    let lb = line_bytes(width, bit_count) as usize;
    let w = width as usize;
    let h = height as usize;
    let mut mat = vec![vec![0u8; w]; h];
    for row in 0..h {
        let bmp_row = if top_down { row } else { h - 1 - row };
        let line_start = off + bmp_row * lb;
        for col in 0..w {
            mat[row][col] = buf[line_start + col];
        }
    }
    mat
}

/// 读取 24 位像素区为 RGB 矩阵（行主序）
pub fn read_pixels_rgb(
    buf: &[u8],
    off: usize,
    width: u32,
    height: u32,
    top_down: bool,
) -> Vec<Vec<(u8, u8, u8)>> {
    let lb = line_bytes(width, 24) as usize;
    let w = width as usize;
    let h = height as usize;
    let mut mat = vec![vec![(0u8, 0u8, 0u8); w]; h];
    for row in 0..h {
        let bmp_row = if top_down { row } else { h - 1 - row };
        let line_start = off + bmp_row * lb;
        for col in 0..w {
            let p = line_start + col * 3;
            // BMP 顺序为 B, G, R
            mat[row][col] = (buf[p + 2], buf[p + 1], buf[p]);
        }
    }
    mat
}

/// 将 2D 像素矩阵写回 BMP 像素区（自下而上）
pub fn write_pixels(buf: &mut [u8], off: usize, mat: &[Vec<u8>], width: u32, bit_count: u16) {
    let lb = line_bytes(width, bit_count) as usize;
    let w = width as usize;
    let h = mat.len();
    for row in 0..h {
        let bmp_row = h - 1 - row;
        let line_start = off + bmp_row * lb;
        for col in 0..w {
            buf[line_start + col] = mat[row][col];
        }
    }
}

/// 简单量化 RGB 像素到 256 级灰度，返回 8 位 BMP 像素矩阵
pub fn quantize_to_gray(rgb: &[Vec<(u8, u8, u8)>]) -> Vec<Vec<u8>> {
    let mut out = Vec::with_capacity(rgb.len());
    for row in rgb {
        let mut r = Vec::with_capacity(row.len());
        for &(r8, g8, b8) in row {
            // 加权灰度：用整数避免浮点临界值带来的 1 级误差
            let y = ((299 * r8 as u32 + 587 * g8 as u32 + 114 * b8 as u32) / 1000) as u8;
            r.push(y);
        }
        out.push(r);
    }
    out
}

/// 从文件读取整个 BMP
pub fn read_file<P: AsRef<Path>>(
    path: P,
) -> io::Result<(FileHeader, InfoHeader, Vec<RgbQuad>, Vec<Vec<u8>>)> {
    let bytes = fs::read(path)?;
    let file_header = read_file_header(&bytes)?;
    if file_header.bf_type != *b"BM" {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "不是合法的 BMP 文件（缺少 BM 标志）",
        ));
    }
    let info_header = read_info_header(&bytes, 14)?;
    if info_header.bi_compression != 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "暂不支持压缩的 BMP",
        ));
    }

    let width = info_header.bi_width.unsigned_abs();
    let height = info_header.bi_height.unsigned_abs();
    let top_down = info_header.bi_height < 0;
    let pixels_off = file_header.bf_off_bits as usize;

    if info_header.bi_bit_count == 24 {
        // 24 位真彩色：无调色板，临时转灰度索引
        let rgb = read_pixels_rgb(&bytes, pixels_off, width, height, top_down);
        let pixels = quantize_to_gray(&rgb);
        let palette: Vec<RgbQuad> = (0..=255u8)
            .map(|i| RgbQuad {
                blue: i,
                green: i,
                red: i,
                reserved: 0,
            })
            .collect();
        let mut info = info_header.clone();
        info.bi_bit_count = 8;
        info.bi_size_image = (line_bytes(width, 8) * height) as u32;
        return Ok((file_header, info, palette, pixels));
    }

    let palette_off = 14 + info_header.bi_size as usize;
    let palette_count = if info_header.bi_clr_used == 0 {
        1usize << info_header.bi_bit_count
    } else {
        info_header.bi_clr_used as usize
    };
    let palette = read_palette(&bytes, palette_off, palette_count);
    let pixels = read_pixels(
        &bytes,
        pixels_off,
        width,
        height,
        info_header.bi_bit_count,
        top_down,
    );

    Ok((file_header, info_header, palette, pixels))
}

/// 将调色板 + 像素 + 文件头信息组装为完整 BMP 字节
pub fn build_bytes(info_template: &InfoHeader, palette: &[RgbQuad], pixels: &[Vec<u8>]) -> Vec<u8> {
    let width = pixels[0].len() as u32;
    let height = pixels.len() as u32;
    let bit_count = info_template.bi_bit_count;
    let lb = line_bytes(width, bit_count);
    let pixel_bytes = (lb * height) as usize;
    let palette_bytes = palette.len() * 4;
    let off_bits = 14 + info_template.bi_size as usize + palette_bytes;
    let total = 14 + info_template.bi_size as usize + palette_bytes + pixel_bytes;

    let mut buf = vec![0u8; total];

    // 文件头
    let fh = FileHeader {
        bf_type: *b"BM",
        bf_size: total as u32,
        bf_reserved1: 0,
        bf_reserved2: 0,
        bf_off_bits: off_bits as u32,
    };
    write_file_header(&mut buf, &fh);

    // 信息头：覆盖宽、高、行字节、像素区大小
    let mut info = info_template.clone();
    info.bi_width = width as i32;
    info.bi_height = height as i32;
    info.bi_size_image = pixel_bytes as u32;
    write_info_header(&mut buf, 14, &info);

    // 调色板
    write_palette(&mut buf, 14 + info.bi_size as usize, palette);

    // 像素区
    write_pixels(&mut buf, off_bits, pixels, width, bit_count);
    buf
}

/// 将完整 BMP 字节写入文件
pub fn write_file<P: AsRef<Path>>(
    path: P,
    info_template: &InfoHeader,
    palette: &[RgbQuad],
    pixels: &[Vec<u8>],
) -> io::Result<()> {
    let bytes = build_bytes(info_template, palette, pixels);
    fs::write(path, bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_bytes_8bit_width_4() {
        // 4 像素 * 8bit / 8 = 4 bytes, 已 4 对齐
        assert_eq!(line_bytes(4, 8), 4);
    }

    #[test]
    fn line_bytes_8bit_width_5() {
        // 5 像素：ceil(5/4)*4 = 8
        assert_eq!(line_bytes(5, 8), 8);
    }

    #[test]
    fn read_pixels_flips_rows() {
        // width=3 时 line_bytes=4，2 行共 8 字节
        // BMP 存储：第 1 行（图像底）在前，第 0 行（图像顶）在后
        let mut buf = vec![0u8; 8];
        buf[0..3].copy_from_slice(&[10, 11, 12]); // 底行（行末填充 0）
        buf[4..7].copy_from_slice(&[20, 21, 22]); // 顶行
        let mat = read_pixels(&buf, 0, 3, 2, 8, false);
        assert_eq!(mat[0], vec![20, 21, 22]);
        assert_eq!(mat[1], vec![10, 11, 12]);
    }

    #[test]
    fn read_pixels_top_down_keeps_rows() {
        // biHeight 为负时，BMP 文件第一行就是图像顶行
        let mut buf = vec![0u8; 8];
        buf[0..3].copy_from_slice(&[20, 21, 22]);
        buf[4..7].copy_from_slice(&[10, 11, 12]);
        let mat = read_pixels(&buf, 0, 3, 2, 8, true);
        assert_eq!(mat[0], vec![20, 21, 22]);
        assert_eq!(mat[1], vec![10, 11, 12]);
    }

    #[test]
    fn write_pixels_flips_rows_back() {
        let mat = vec![vec![20u8, 21, 22], vec![10, 11, 12]];
        let mut buf = vec![0u8; 8];
        write_pixels(&mut buf, 0, &mat, 3, 8);
        // 写入后第 0 行（图像顶）应在底部
        assert_eq!(&buf[0..3], &[10, 11, 12]);
        assert_eq!(&buf[4..7], &[20, 21, 22]);
    }
}
