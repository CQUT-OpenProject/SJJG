use ex_5::{build_tree, sample_data};

fn main() {
    let data = sample_data();
    let tree = build_tree(&data);

    println!("输入数据:");
    for x in &data {
        print!("{:4}", x);
    }
    println!();

    println!("二叉排序树中序序列:");
    for x in tree.inorder() {
        print!("{:4}", x);
    }
    println!();

    println!("查找 37: {}", tree.search(37));
    println!("查找 100: {}", tree.search(100));
}
