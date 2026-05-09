use std::fs;

use ex_1::{build_tree, inorder_string};

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("inputfile.txt")?;
    let root = build_tree(input.trim());
    let result = inorder_string(&root);

    println!("输入字符串: {}", input.trim());
    println!("中序遍历结果: {}", result);

    Ok(())
}
