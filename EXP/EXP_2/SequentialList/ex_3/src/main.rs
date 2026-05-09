use seq_ex_3::find_min_max;
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

    match find_min_max(&numbers[1..=n]) {
        Some(result) => {
            println!("最小值: {}", result.min);
            println!("最大值: {}", result.max);
            println!("比较次数: {}", result.comparisons);
        }
        None => println!("顺序表为空"),
    }

    Ok(())
}
