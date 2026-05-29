use std::io;

use ex_4::{insert_ordered, sample_data};

fn main() {
    let mut data = sample_data();

    println!("原有序表:");
    for x in &data {
        print!("{:3}", x);
    }
    println!();

    println!("输入要插入的元素:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let x = input.trim().parse::<i32>().unwrap_or(0);

    let pos = insert_ordered(&mut data, x);
    println!("插入位置: {}", pos);
    println!("插入后的有序表:");
    for item in &data {
        print!("{:3}", item);
    }
    println!();
}
