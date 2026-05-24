use std::fs::File;
use std::io::{BufRead, BufReader};

use ex_1::fun;

fn main() -> std::io::Result<()> {
    let file = File::open("inputfile.txt")?;
    let reader = BufReader::new(file);

    let mut numbers = Vec::new(); // 数据大小未知，先在堆上分配

    for line in reader.lines() {
        for word in line?.split_whitespace() {
            if let Ok(num) = word.parse::<i32>() {
                numbers.push(num); // 入栈
            }
        }
    }

    let n = numbers[0] as usize; // 假设文件第一个数是元素个数
    let data = &numbers[1..=n]; // 使用切片避免所有权转移

    let mut odd_sum = 0;
    let mut even_sum = 0;

    fun(data, &mut odd_sum, &mut even_sum);

    println!("奇数之和: {}", odd_sum);
    println!("偶数之和: {}", even_sum);

    Ok(())
}
