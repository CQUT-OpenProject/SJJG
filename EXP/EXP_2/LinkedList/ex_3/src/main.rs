use link_ex_3::CircularList;
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

    let mut list = CircularList::new();
    for key in numbers {
        list.add_tail(key);
    }

    println!("单循环链表一轮输出: {:?}", list.output());
    println!("循环输出 8 个元素: {:?}", list.circular_output(8));
    println!("第 2 个元素: {:?}", list.get(2));
    println!("搜索 3: {:?}", list.search(3));
    list.add_head(100);
    list.add_tail(200);
    println!("插入后: {:?}", list.output());
    println!("删除第 2 个元素: {:?}", list.delete_at(2));
    println!("删除键值 200: {:?}", list.delete_key(200));
    println!("删除后: {:?}", list.output());

    Ok(())
}
