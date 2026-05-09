use ex_1::{build_tree, inorder_string};

#[test]
fn builds_binary_search_tree_from_chars() {
    let root = build_tree("ephqsbma");
    assert_eq!(inorder_string(&root), "abehmpqs");
}

#[test]
fn ignores_spaces_when_building_tree() {
    let root = build_tree("e p h q s b m a");
    assert_eq!(inorder_string(&root), "abehmpqs");
}

#[test]
fn empty_input_gives_empty_output() {
    let root = build_tree("");
    assert_eq!(inorder_string(&root), "");
}
