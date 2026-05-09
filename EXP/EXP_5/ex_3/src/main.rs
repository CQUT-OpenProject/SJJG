use ex_3::{sample_seq_table, sample_tree};

fn main() {
    let table = sample_seq_table();
    let tree = sample_tree();

    println!("顺序表: {:?}", table);
    println!("层次遍历: {:?}", tree.level_order());
    println!("先序遍历: {:?}", tree.preorder());
    println!("中序遍历: {:?}", tree.inorder());
    println!("后序遍历: {:?}", tree.postorder());
    println!("树深度: {}", tree.depth());
}
