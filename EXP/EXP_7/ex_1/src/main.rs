use std::io;

use ex_1::{sample_data, search};

fn main() {
    let a = sample_data();
    let mut i = 0;

    println!("A 数值");
    print!("下标");
    while i < a.len() {
        print!("{:3}", i);
        i += 1;
    }
    println!();

    i = 0;
    print!("值  ");
    while i < a.len() {
        print!("{:3}", a[i]);
        i += 1;
    }
    println!();

    println!("输入值:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let d = input.trim().parse::<i32>().unwrap_or(0);

    let n = search(&a, d);
    if n >= 0 {
        println!("A[{}]={}", n, d);
    } else {
        println!("{} 未找到", d);
    }
}
