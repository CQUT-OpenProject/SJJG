use std::fs::File;
use std::io::{BufRead, BufReader};

use ex_2::exchange;

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

	let data = &mut numbers[1..=n];
	if exchange(data) {
		println!("交换后数组: {:?}", data);
	} else {
		println!("err");
	}

	Ok(())
}

