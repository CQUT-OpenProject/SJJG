use std::env;
use std::process;

use exp_4::{process_dir, process_file};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!(
            "用法:\n  {} <输入bmp> <输出目录>\n  {} --dir <输入目录> <输出目录>",
            args[0], args[0]
        );
        process::exit(1);
    }

    if args[1] == "--dir" {
        if args.len() < 4 {
            eprintln!("用法: {} --dir <输入目录> <输出目录>", args[0]);
            process::exit(1);
        }
        let count = process_dir(&args[2], &args[3]).unwrap_or_else(|e| {
            eprintln!("处理目录失败: {}", e);
            process::exit(1);
        });
        println!("已处理 {} 个 BMP 文件", count);
    } else {
        process_file(&args[1], &args[2]).unwrap_or_else(|e| {
            eprintln!("处理文件失败: {}", e);
            process::exit(1);
        });
        println!("已写入水平镜像、垂直镜像、转置结果");
    }
}
