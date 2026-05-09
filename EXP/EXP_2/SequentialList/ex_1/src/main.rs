use seq_ex_1::SeqList;
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

    if numbers.len() < 2 {
        println!("输入数据不足");
        return Ok(());
    }

    let max_len = numbers[0] as usize;
    let n = numbers[1] as usize;
    if numbers.len() < n + 2 {
        println!("输入数据不足");
        return Ok(());
    }

    let mut list = SeqList::new(max_len);
    for &key in &numbers[2..2 + n] {
        list.add_tail(key);
    }

    println!("线性表: {:?}", list.output());
    println!("第 2 个元素: {:?}", list.get(2));
    println!("键值 {} 的位置: {:?}", numbers[2], list.search(numbers[2]));

    list.add_head(100);
    list.add_tail(200);
    list.insert(2, 150);
    println!("插入后: {:?}", list.output());

    println!("删除第 2 个元素: {:?}", list.delete_at(2));
    println!("删除键值 200: {:?}", list.delete_key(200));
    println!("删除后: {:?}", list.output());

    Ok(())
}
