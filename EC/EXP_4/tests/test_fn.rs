use std::fs;
use std::io::Write;

use exp_4::{Image, InfoHeader, RgbQuad, build_bytes, process_dir, process_file};

/// 手工写一张 3x2 灰度 BMP，便于单元测试
fn write_sample_bmp(path: &str, w: u32, h: u32, mat: &[Vec<u8>]) {
    let palette: Vec<RgbQuad> = (0..=255u8)
        .map(|i| RgbQuad {
            blue: i,
            green: i,
            red: i,
            reserved: 0,
        })
        .collect();
    let info = InfoHeader {
        bi_size: 40,
        bi_width: w as i32,
        bi_height: h as i32,
        bi_planes: 1,
        bi_bit_count: 8,
        bi_compression: 0,
        bi_size_image: 0,
        bi_x_pels_per_meter: 0,
        bi_y_pels_per_meter: 0,
        bi_clr_used: 256,
        bi_clr_important: 0,
    };
    let bytes = build_bytes(&info, &palette, mat);
    let mut f = fs::File::create(path).unwrap();
    f.write_all(&bytes).unwrap();
}

/// 手工写一张 24 位 BMP。top_down=true 时 biHeight 写为负数。
fn write_sample_24bit_bmp(path: &str, top_down: bool) {
    let w = 2u32;
    let h = 2u32;
    let lb = (w * 3 + 3) & !3;
    let off_bits = 14 + 40;
    let total = off_bits + (lb * h) as usize;
    let mut buf = vec![0u8; total];

    buf[0] = b'B';
    buf[1] = b'M';
    buf[2..6].copy_from_slice(&(total as u32).to_le_bytes());
    buf[10..14].copy_from_slice(&(off_bits as u32).to_le_bytes());

    buf[14..18].copy_from_slice(&40u32.to_le_bytes());
    buf[18..22].copy_from_slice(&w.to_le_bytes());
    let signed_h = if top_down { -(h as i32) } else { h as i32 };
    buf[22..26].copy_from_slice(&signed_h.to_le_bytes());
    buf[26..28].copy_from_slice(&1u16.to_le_bytes());
    buf[28..30].copy_from_slice(&24u16.to_le_bytes());

    // 图像顶行：蓝、白；底行：红、绿。每个像素按 BGR 写入。
    let rows: [[u8; 6]; 2] = [[255, 0, 0, 255, 255, 255], [0, 0, 255, 0, 255, 0]];
    for row in 0..h as usize {
        let file_row = if top_down { row } else { h as usize - 1 - row };
        let off = off_bits + file_row * lb as usize;
        buf[off..off + (w as usize) * 3].copy_from_slice(&rows[row]);
    }

    fs::write(path, &buf).unwrap();
}

#[test]
fn mirror_horizontal_reverses_columns() {
    let img = Image::make_solid(3, 2, 1);
    let mut pixels = img.pixels.clone();
    // 设置非均匀值便于观察
    pixels[0] = vec![10, 20, 30];
    pixels[1] = vec![40, 50, 60];
    let img2 = Image { pixels, ..img };
    let mh = img2.mirror_horizontal();
    assert_eq!(mh.pixels[0], vec![30, 20, 10]);
    assert_eq!(mh.pixels[1], vec![60, 50, 40]);
    assert_eq!(mh.width, 3);
    assert_eq!(mh.height, 2);
}

#[test]
fn mirror_vertical_reverses_rows() {
    let img = Image::make_solid(2, 3, 1);
    let mut pixels = img.pixels.clone();
    pixels[0] = vec![10, 20];
    pixels[1] = vec![30, 40];
    pixels[2] = vec![50, 60];
    let img2 = Image { pixels, ..img };
    let mv = img2.mirror_vertical();
    assert_eq!(mv.pixels[0], vec![50, 60]);
    assert_eq!(mv.pixels[1], vec![30, 40]);
    assert_eq!(mv.pixels[2], vec![10, 20]);
}

#[test]
fn transpose_swaps_dimensions() {
    let img = Image::make_solid(2, 3, 1);
    let mut pixels = img.pixels.clone();
    pixels[0] = vec![10, 20];
    pixels[1] = vec![30, 40];
    pixels[2] = vec![50, 60];
    let img2 = Image { pixels, ..img };
    let t = img2.transpose();
    // 新图宽 3 高 2：原 (row, col) -> 新 (col, row)
    assert_eq!(t.width, 3);
    assert_eq!(t.height, 2);
    assert_eq!(t.pixels[0], vec![10, 30, 50]);
    assert_eq!(t.pixels[1], vec![20, 40, 60]);
}

#[test]
fn double_mirror_horizontal_returns_original() {
    let img = Image::make_solid(4, 3, 1);
    let mut pixels = img.pixels.clone();
    for r in 0..3 {
        for c in 0..4 {
            pixels[r][c] = (r * 4 + c) as u8;
        }
    }
    let img2 = Image { pixels, ..img };
    let orig_pixels = img2.pixels.clone();
    let twice = img2.mirror_horizontal().mirror_horizontal();
    assert_eq!(twice.pixels, orig_pixels);
}

#[test]
fn double_mirror_vertical_returns_original() {
    let img = Image::make_solid(4, 3, 1);
    let mut pixels = img.pixels.clone();
    for r in 0..3 {
        for c in 0..4 {
            pixels[r][c] = (r * 4 + c) as u8;
        }
    }
    let img2 = Image { pixels, ..img };
    let orig_pixels = img2.pixels.clone();
    let twice = img2.mirror_vertical().mirror_vertical();
    assert_eq!(twice.pixels, orig_pixels);
}

#[test]
fn double_transpose_returns_original() {
    let img = Image::make_solid(4, 3, 1);
    let mut pixels = img.pixels.clone();
    for r in 0..3 {
        for c in 0..4 {
            pixels[r][c] = (r * 4 + c) as u8;
        }
    }
    let img2 = Image { pixels, ..img };
    let orig = img2.clone();
    let twice = img2.transpose().transpose();
    assert_eq!(twice.width, orig.width);
    assert_eq!(twice.height, orig.height);
    assert_eq!(twice.pixels, orig.pixels);
}

#[test]
fn bmp_roundtrip_preserves_pixels() {
    let dir = std::env::temp_dir().join("exp_4_bmp_roundtrip");
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();

    let mat = vec![vec![1u8, 2, 3], vec![4, 5, 6]];
    let path = dir.join("sample.bmp");
    write_sample_bmp(path.to_str().unwrap(), 3, 2, &mat);

    let img = Image::read(&path).unwrap();
    assert_eq!(img.width, 3);
    assert_eq!(img.height, 2);
    assert_eq!(img.pixels, mat);
    assert_eq!(img.palette.len(), 256);
}

#[test]
fn mirror_horizontal_roundtrip_via_file() {
    let dir = std::env::temp_dir().join("exp_4_bmp_mirror");
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();

    let mat = vec![vec![1u8, 2, 3, 4], vec![5, 6, 7, 8]];
    let path = dir.join("orig.bmp");
    write_sample_bmp(path.to_str().unwrap(), 4, 2, &mat);

    let img = Image::read(&path).unwrap();
    let mh = img.mirror_horizontal();
    let out = dir.join("mh.bmp");
    mh.write(&out).unwrap();

    let mh_read = Image::read(&out).unwrap();
    assert_eq!(mh_read.pixels[0], vec![4, 3, 2, 1]);
    assert_eq!(mh_read.pixels[1], vec![8, 7, 6, 5]);
}

#[test]
fn transpose_roundtrip_via_file() {
    let dir = std::env::temp_dir().join("exp_4_bmp_transpose");
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();

    // 2x3 -> 3x2
    let mat = vec![vec![1u8, 2, 3], vec![4, 5, 6]];
    let path = dir.join("orig.bmp");
    write_sample_bmp(path.to_str().unwrap(), 3, 2, &mat);

    let img = Image::read(&path).unwrap();
    let t = img.transpose();
    let out = dir.join("t.bmp");
    t.write(&out).unwrap();

    let t_read = Image::read(&out).unwrap();
    assert_eq!(t_read.width, 2);
    assert_eq!(t_read.height, 3);
    assert_eq!(t_read.pixels[0], vec![1, 4]);
    assert_eq!(t_read.pixels[1], vec![2, 5]);
    assert_eq!(t_read.pixels[2], vec![3, 6]);
}

#[test]
fn mirror_vertical_roundtrip_via_file() {
    let dir = std::env::temp_dir().join("exp_4_bmp_mirror_v");
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();

    let mat = vec![vec![1u8, 2, 3, 4], vec![5, 6, 7, 8]];
    let path = dir.join("orig.bmp");
    write_sample_bmp(path.to_str().unwrap(), 4, 2, &mat);

    let img = Image::read(&path).unwrap();
    let mv = img.mirror_vertical();
    let out = dir.join("mv.bmp");
    mv.write(&out).unwrap();

    let mv_read = Image::read(&out).unwrap();
    assert_eq!(mv_read.pixels[0], vec![5, 6, 7, 8]);
    assert_eq!(mv_read.pixels[1], vec![1, 2, 3, 4]);
}

/// 24 位真彩色 BMP 也能被读入并量化
#[test]
fn read_24bit_bmp_quantizes_to_gray() {
    let dir = std::env::temp_dir().join("exp_4_bmp_24bit");
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();

    // 手写一个最小 24 位 BMP：2x2，红绿蓝白
    let w = 2u32;
    let h = 2u32;
    let lb = (w * 3 + 3) & !3;
    let palette_bytes = 0;
    let off_bits = 14 + 40 + palette_bytes;
    let total = off_bits + (lb * h) as usize;
    let mut buf = vec![0u8; total];

    // 文件头
    buf[0] = b'B';
    buf[1] = b'M';
    buf[2..6].copy_from_slice(&(total as u32).to_le_bytes());
    buf[10..14].copy_from_slice(&(off_bits as u32).to_le_bytes());

    // 信息头
    buf[14..18].copy_from_slice(&40u32.to_le_bytes());
    buf[18..22].copy_from_slice(&w.to_le_bytes());
    buf[22..26].copy_from_slice(&h.to_le_bytes());
    buf[26..28].copy_from_slice(&1u16.to_le_bytes());
    buf[28..30].copy_from_slice(&24u16.to_le_bytes());

    // 像素：BMP 自下而上，1px=3byte BGR
    // 第 1 行（底）：纯红 (0,0,255)、纯绿 (0,255,0)
    // 第 0 行（顶）：纯蓝 (255,0,0)、白 (255,255,255)
    let bmp_pixels: [[u8; 6]; 2] = [
        // 第 0 行（顶）：蓝、白
        [255, 0, 0, 255, 255, 255],
        // 第 1 行（底）：红、绿
        [0, 0, 255, 0, 255, 0],
    ];
    for row in 0..h as usize {
        let bmp_row = h as usize - 1 - row;
        let off = off_bits + bmp_row * lb as usize;
        buf[off..off + (w as usize) * 3].copy_from_slice(&bmp_pixels[row]);
    }

    let path = dir.join("rgb.bmp");
    fs::write(&path, &buf).unwrap();

    let img = Image::read(&path).unwrap();
    assert_eq!(img.width, 2);
    assert_eq!(img.height, 2);
    // 量化到灰度后调色板应为 256 项
    assert_eq!(img.palette.len(), 256);
    // 蓝 (0,0,255) -> 0.114*255 ≈ 29
    assert!(img.pixels[0][0] < 40);
    // 白 -> 255
    assert_eq!(img.pixels[0][1], 255);
    // 红 (255,0,0) -> 0.299*255 ≈ 76
    assert!(img.pixels[1][0] < 90);
    // 绿 (0,255,0) -> 0.587*255 ≈ 150
    assert!(img.pixels[1][1] > 130 && img.pixels[1][1] < 170);
}

#[test]
fn read_top_down_24bit_bmp_keeps_image_rows() {
    let dir = std::env::temp_dir().join("exp_4_bmp_24bit_top_down");
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();

    let path = dir.join("rgb_top_down.bmp");
    write_sample_24bit_bmp(path.to_str().unwrap(), true);

    let img = Image::read(&path).unwrap();
    assert_eq!(img.width, 2);
    assert_eq!(img.height, 2);
    // 顶行仍应是蓝、白；底行仍应是红、绿
    assert!(img.pixels[0][0] < 40);
    assert_eq!(img.pixels[0][1], 255);
    assert!(img.pixels[1][0] < 90);
    assert!(img.pixels[1][1] > 130 && img.pixels[1][1] < 170);
}

#[test]
fn process_file_creates_output_dir_and_writes_three_files() {
    let dir = std::env::temp_dir().join("exp_4_process_file");
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();

    let input = dir.join("one.bmp");
    let out = dir.join("new_out_dir");
    let mat = vec![vec![1u8, 2, 3], vec![4, 5, 6]];
    write_sample_bmp(input.to_str().unwrap(), 3, 2, &mat);

    process_file(&input, &out).unwrap();

    assert!(out.join("one_mirror_h.bmp").exists());
    assert!(out.join("one_mirror_v.bmp").exists());
    assert!(out.join("one_transpose.bmp").exists());
}

#[test]
fn process_dir_writes_three_outputs_for_each_bmp() {
    let dir = std::env::temp_dir().join("exp_4_process_dir");
    let _ = fs::remove_dir_all(&dir);
    let input_dir = dir.join("in");
    let out_dir = dir.join("out");
    fs::create_dir_all(&input_dir).unwrap();

    let mat = vec![vec![1u8, 2, 3], vec![4, 5, 6]];
    for i in 0..5 {
        let path = input_dir.join(format!("pic{}.bmp", i));
        write_sample_bmp(path.to_str().unwrap(), 3, 2, &mat);
    }

    let count = process_dir(&input_dir, &out_dir).unwrap();
    let output_count = fs::read_dir(&out_dir)
        .unwrap()
        .filter(|entry| {
            entry
                .as_ref()
                .unwrap()
                .path()
                .extension()
                .and_then(|s| s.to_str())
                == Some("bmp")
        })
        .count();

    assert_eq!(count, 5);
    assert_eq!(output_count, 15);
}
