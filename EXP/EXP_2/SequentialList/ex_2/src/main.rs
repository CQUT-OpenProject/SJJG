use seq_ex_2::split_positive_negative;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("inputfile.txt")?;
    let reader = BufReader::new(file);

    let mut numbers = Vec::new();
    for line in reader.lines() {
        for word in line?.split_whitespace() {
            if let Ok(num) = word.parse::<i32>() {
                numbers.push(num);
            }
        }
    }

    if numbers.is_empty() {
        println!("输入为空");
        return Ok(());
    }

    let n = numbers[0] as usize;
    if numbers.len() < n + 1 {
        println!("输入数据不足");
        return Ok(());
    }

    let (b, c) = split_positive_negative(&numbers[1..=n]);
    println!("顺序表 A: {:?}", &numbers[1..=n]);
    println!("大于 0 的顺序表 B: {:?}", b);
    println!("小于 0 的顺序表 C: {:?}", c);

    Ok(())
}
