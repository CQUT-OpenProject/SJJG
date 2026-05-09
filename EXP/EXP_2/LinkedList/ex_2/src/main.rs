use link_ex_2::SinglyList;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("inputfile.txt")?;
    let reader = BufReader::new(file);

    let mut chars = Vec::new();
    for line in reader.lines() {
        for word in line?.split_whitespace() {
            if let Some(ch) = word.chars().next() {
                chars.push(ch);
            }
        }
    }

    if chars.is_empty() {
        println!("输入为空");
        return Ok(());
    }

    let mut list = SinglyList::new();
    for ch in chars {
        list.add_tail(ch);
    }

    println!("单链表: {:?}", list.output());
    println!("第 2 个元素: {:?}", list.get(2));
    println!("搜索 a: {:?}", list.search('a'));
    list.add_head('x');
    list.add_tail('z');
    println!("插入后: {:?}", list.output());
    println!("删除第 2 个元素: {:?}", list.delete_at(2));
    println!("删除键值 z: {:?}", list.delete_key('z'));
    println!("删除后: {:?}", list.output());

    Ok(())
}
