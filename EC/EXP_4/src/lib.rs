pub mod bmp;
pub mod image;

pub use bmp::{
    FileHeader, InfoHeader, RgbQuad, build_bytes, line_bytes, read_file, read_file_header,
    read_info_header, read_palette, read_pixels, write_file, write_file_header, write_info_header,
    write_palette, write_pixels,
};

pub use image::{Image, process_dir, process_file};
