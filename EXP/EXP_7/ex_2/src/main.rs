use std::io;

use ex_2::{bin_search, sample_data};

fn main() {
    let a = sample_data();
    let mut i = 0;

    println!("有序表中的数据:");
    while i < a.len() {
        print!("{:3}", a[i]);
        i += 1;
    }
    println!();

    println!("输入要查找的键值:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let x = input.trim().parse::<i32>().unwrap_or(0);

    let pos = bin_search(&a, x);
    if pos >= 0 {
        println!("查找成功，位置为 {}", pos);
    } else {
        println!("查找失败");
    }
}
