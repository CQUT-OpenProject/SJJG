use std::io::{self, Write};
use std::process;

use exp_7::{to_html, InvertedIndex};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("用法: {} <文档目录路径>", args[0]);
        eprintln!("  文档目录中应包含 .txt 文本文件");
        eprintln!("  运行时输入关键词进行搜索，输入 :q 退出");
        process::exit(1);
    }

    let dir = &args[1];

    let index = InvertedIndex::build_from_dir(dir).unwrap_or_else(|err| {
        eprintln!("无法构建索引: {}", err);
        process::exit(1);
    });

    println!("=== MARS 小马尔斯 · 搜索引擎 ===");
    println!("索引目录: {}", dir);
    println!("词典大小: {} 个词条", index.term_count());
    println!();
    println!("请输入要搜索的关键词（输入 :q 退出）:");

    loop {
        print!("> ");
        io::stdout().flush().ok();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }

        let keyword = input.trim();

        if keyword.is_empty() {
            continue;
        }

        if keyword == ":q" {
            println!("再见！");
            break;
        }

        let results = index.search(keyword);

        if results.is_empty() {
            println!("（无匹配结果）");
        } else {
            println!("找到 {} 个相关文档:", results.len());
            for doc in &results {
                println!("  · {}.txt", doc);
            }

            let html = to_html(keyword, &results);
            let html_path = "index.html";
            if let Ok(mut file) = std::fs::File::create(html_path) {
                file.write_all(html.as_bytes()).ok();
                println!("  结果已输出到 {}", html_path);
            }
        }
        println!();
    }
}
