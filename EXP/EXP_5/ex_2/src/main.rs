use ex_2::BinaryTree;

fn main() {
    let tree = BinaryTree::sample_tree();

    println!("结点数: {}", tree.count_nodes());
    println!("叶子数: {}", tree.count_leaves());
    println!("层次遍历: {:?}", tree.level_order());
    println!("先序遍历: {:?}", tree.preorder());
    println!("中序遍历: {:?}", tree.inorder());
    println!("后序遍历: {:?}", tree.postorder());
    println!("二叉树深度: {}", tree.depth());

    match tree.subtree_depth(4) {
        Some(depth) => println!("以 4 为根的子树深度: {}", depth),
        None => println!("未找到结点 4"),
    }
}
