use std::fs;
use std::io;
use std::path::Path;

use crate::bmp;

/// 一幅 256 色 BMP 图像
#[derive(Clone, Debug)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    /// 像素矩阵，pixels[row][col]，row=0 对应图像顶行
    pub pixels: Vec<Vec<u8>>,
    pub palette: Vec<bmp::RgbQuad>,
    pub info_header: bmp::InfoHeader,
}

impl Image {
    /// 从 BMP 文件读取
    pub fn read<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let (_file_header, info_header, palette, pixels) = bmp::read_file(path)?;
        let width = info_header.bi_width.unsigned_abs();
        let height = info_header.bi_height.unsigned_abs();
        Ok(Image {
            width,
            height,
            pixels,
            palette,
            info_header,
        })
    }

    /// 写入 BMP 文件
    pub fn write<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        bmp::write_file(path, &self.info_header, &self.palette, &self.pixels)
    }

    /// 水平镜像：左右翻转
    pub fn mirror_horizontal(&self) -> Self {
        let mut img = self.clone();
        for row in 0..self.height as usize {
            img.pixels[row].reverse();
        }
        img
    }

    /// 垂直镜像：上下翻转
    pub fn mirror_vertical(&self) -> Self {
        let mut img = self.clone();
        img.pixels.reverse();
        img
    }

    /// 转置：图像沿主对角线翻转，宽高互换
    /// 原图宽 w、高 h，新图宽 h、高 w
    /// 原图 (row, col) -> 新图 (col, row)
    pub fn transpose(&self) -> Self {
        let w = self.width as usize;
        let h = self.height as usize;
        let mut new_pixels = vec![vec![0u8; h]; w];
        for row in 0..h {
            for col in 0..w {
                new_pixels[col][row] = self.pixels[row][col];
            }
        }

        // 信息头宽高互换
        let mut new_info = self.info_header.clone();
        new_info.bi_width = h as i32;
        new_info.bi_height = w as i32;

        Image {
            width: h as u32,
            height: w as u32,
            pixels: new_pixels,
            palette: self.palette.clone(),
            info_header: new_info,
        }
    }

    /// 生成一张纯色小图，便于单元测试
    pub fn make_solid(width: u32, height: u32, value: u8) -> Self {
        let pixels = vec![vec![value; width as usize]; height as usize];
        let mut palette = Vec::with_capacity(256);
        for i in 0..=255u8 {
            palette.push(bmp::RgbQuad {
                blue: i,
                green: i,
                red: i,
                reserved: 0,
            });
        }
        let info_header = bmp::InfoHeader {
            bi_size: 40,
            bi_width: width as i32,
            bi_height: height as i32,
            bi_planes: 1,
            bi_bit_count: 8,
            bi_compression: 0,
            bi_size_image: 0,
            bi_x_pels_per_meter: 0,
            bi_y_pels_per_meter: 0,
            bi_clr_used: 256,
            bi_clr_important: 0,
        };
        Image {
            width,
            height,
            pixels,
            palette,
            info_header,
        }
    }
}

/// 处理一张 BMP 文件：将水平镜像、垂直镜像、转置结果写到 out_dir 下
/// 输入文件名 basename 会作为结果文件名前缀
pub fn process_file<P: AsRef<Path>, Q: AsRef<Path>>(input: P, out_dir: Q) -> io::Result<()> {
    fs::create_dir_all(&out_dir)?;
    let img = Image::read(&input)?;
    let input_name = input
        .as_ref()
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("image")
        .to_string();

    let mh = img.mirror_horizontal();
    mh.write(
        out_dir
            .as_ref()
            .join(format!("{}_mirror_h.bmp", input_name)),
    )?;

    let mv = img.mirror_vertical();
    mv.write(
        out_dir
            .as_ref()
            .join(format!("{}_mirror_v.bmp", input_name)),
    )?;

    let tr = img.transpose();
    tr.write(
        out_dir
            .as_ref()
            .join(format!("{}_transpose.bmp", input_name)),
    )?;

    Ok(())
}

/// 读取一个目录下的全部 .bmp，分别处理；返回处理文件个数
pub fn process_dir<P: AsRef<Path>, Q: AsRef<Path>>(input_dir: P, out_dir: Q) -> io::Result<usize> {
    fs::create_dir_all(&out_dir)?;
    let mut count = 0;
    for entry in fs::read_dir(&input_dir)? {
        let entry = entry?;
        let path = entry.path();
        let is_bmp = path
            .extension()
            .and_then(|s| s.to_str())
            .map(|s| s.eq_ignore_ascii_case("bmp"))
            .unwrap_or(false);
        if is_bmp {
            process_file(&path, &out_dir)?;
            count += 1;
        }
    }
    Ok(count)
}
