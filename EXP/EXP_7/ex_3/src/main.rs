use std::io;

use ex_3::{block_search, sample_data, sample_index};

fn main() {
    let data = sample_data();
    let index = sample_index();

    println!("分块表数据:");
    let mut i = 0;
    while i < data.len() {
        print!("{:4}", data[i]);
        i += 1;
    }
    println!();

    println!("索引表:");
    for item in &index {
        println!("start = {:2}, max_key = {}", item.start, item.max_key);
    }

    println!("输入要查找的关键字:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let key = input.trim().parse::<i32>().unwrap_or(0);

    let pos = block_search(&data, &index, 6, key);
    if pos >= 0 {
        println!("查找成功，位置为 {}", pos);
    } else {
        println!("查找失败");
    }
}
