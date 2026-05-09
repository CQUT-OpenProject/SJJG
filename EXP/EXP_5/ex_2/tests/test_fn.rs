use ex_2::BinaryTree;

#[test]
fn counts_nodes_and_leaves() {
    let tree = BinaryTree::sample_tree();

    assert_eq!(tree.count_nodes(), 10);
    assert_eq!(tree.count_leaves(), 5);
}

#[test]
fn traverses_tree_in_all_orders() {
    let tree = BinaryTree::sample_tree();

    assert_eq!(tree.level_order(), vec![10, 3, 18, 2, 4, 13, 21, 9, 8, 9]);
    assert_eq!(tree.preorder(), vec![10, 3, 2, 4, 9, 8, 9, 18, 13, 21]);
    assert_eq!(tree.inorder(), vec![2, 3, 4, 8, 9, 9, 10, 13, 18, 21]);
    assert_eq!(tree.postorder(), vec![2, 8, 9, 9, 4, 3, 13, 21, 18, 10]);
}

#[test]
fn computes_tree_depth_and_subtree_depth() {
    let tree = BinaryTree::sample_tree();

    assert_eq!(tree.depth(), 5);
    assert_eq!(tree.subtree_depth(4), Some(3));
    assert_eq!(tree.subtree_depth(18), Some(2));
    assert_eq!(tree.subtree_depth(100), None);
}
