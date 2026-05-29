use ex_5::{BinaryTree, build_tree, sample_data};

#[test]
fn build_tree_gets_ordered_inorder_result() {
    let data = sample_data();
    let tree = build_tree(&data);

    assert_eq!(tree.inorder(), vec![12, 24, 28, 37, 40, 45, 53, 60, 70, 93]);
}

#[test]
fn tree_search_finds_existing_data() {
    let data = sample_data();
    let tree = build_tree(&data);

    assert!(tree.search(45));
    assert!(tree.search(28));
    assert!(tree.search(70));
}

#[test]
fn tree_rejects_duplicate_data() {
    let mut tree = BinaryTree::new();

    assert!(tree.insert(10));
    assert!(!tree.insert(10));
    assert_eq!(tree.inorder(), vec![10]);
}
