use ex_4::{ChildTree, sample_tree};

#[test]
fn finds_all_children_in_insert_order() {
    let tree = sample_tree();

    assert_eq!(tree.child_values('A'), Some(vec!['B', 'C', 'D']));
    assert_eq!(tree.child_values('B'), Some(vec!['E', 'F']));
    assert_eq!(tree.child_values('D'), Some(vec!['G']));
}

#[test]
fn leaf_node_has_no_children() {
    let tree = sample_tree();

    assert_eq!(tree.child_values('G'), Some(Vec::<char>::new()));
}

#[test]
fn rejects_missing_parent_and_duplicate_child() {
    let mut tree = ChildTree::new('A');

    assert!(!tree.add_child('X', 'B'));
    assert!(tree.add_child('A', 'B'));
    assert!(!tree.add_child('A', 'B'));
    assert_eq!(tree.child_values('Z'), None);
}
