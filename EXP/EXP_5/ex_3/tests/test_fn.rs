use ex_3::{BinaryTree, sample_seq_table, sample_tree};

#[test]
fn builds_binary_tree_from_sequence_table() {
    let tree = sample_tree();

    assert_eq!(tree.level_order(), vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H']);
}

#[test]
fn keeps_expected_traversal_results() {
    let tree = sample_tree();

    assert_eq!(tree.preorder(), vec!['A', 'B', 'D', 'G', 'C', 'E', 'H', 'F']);
    assert_eq!(tree.inorder(), vec!['D', 'G', 'B', 'A', 'H', 'E', 'C', 'F']);
    assert_eq!(tree.postorder(), vec!['G', 'D', 'B', 'H', 'E', 'F', 'C', 'A']);
    assert_eq!(tree.depth(), 4);
}

#[test]
fn empty_slot_stays_empty() {
    let table = sample_seq_table();
    let tree = BinaryTree::from_seq_table(&table);

    assert_eq!(tree.level_order(), vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H']);
}
