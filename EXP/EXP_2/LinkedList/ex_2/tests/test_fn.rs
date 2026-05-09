use link_ex_2::SinglyList;

#[test]
fn singly_list_supports_table_10_7_operations() {
    let mut list = SinglyList::new();

    assert_eq!(list.output(), Vec::<char>::new());
    assert!(list.add_tail('b'));
    assert!(list.add_head('a'));
    assert!(list.insert(3, 'd'));
    assert!(list.insert(3, 'c'));

    assert_eq!(list.output(), vec!['a', 'b', 'c', 'd']);
    assert_eq!(list.get(2), Some('b'));
    assert_eq!(list.search('c'), Some(3));
    assert_eq!(list.search('x'), None);
    assert_eq!(list.delete_at(2), Some('b'));
    assert_eq!(list.delete_key('d'), Some('d'));
    assert_eq!(list.output(), vec!['a', 'c']);
}

#[test]
fn singly_list_rejects_duplicate_and_invalid_positions() {
    let mut list = SinglyList::new();

    assert!(!list.insert(0, 'a'));
    assert!(list.add_tail('a'));
    assert!(!list.add_head('a'));
    assert!(!list.insert(3, 'b'));
    assert_eq!(list.delete_at(2), None);
    assert_eq!(list.output(), vec!['a']);
}
