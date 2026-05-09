use link_ex_4::DoublyList;

#[test]
fn doubly_list_supports_table_10_7_operations() {
    let mut list = DoublyList::new();

    assert!(list.add_tail(20));
    assert!(list.add_head(10));
    assert!(list.insert(2, 15));
    assert!(list.add_tail(30));

    assert_eq!(list.output(), vec![10, 15, 20, 30]);
    assert_eq!(list.reverse_output(), vec![30, 20, 15, 10]);
    assert_eq!(list.get(3), Some(20));
    assert_eq!(list.search(15), Some(2));
    assert_eq!(list.delete_at(1), Some(10));
    assert_eq!(list.delete_key(30), Some(30));
    assert_eq!(list.output(), vec![15, 20]);
    assert_eq!(list.reverse_output(), vec![20, 15]);
}

#[test]
fn doubly_list_rejects_duplicates_and_bad_positions() {
    let mut list = DoublyList::new();

    assert!(!list.insert(0, 1));
    assert!(list.add_tail(1));
    assert!(!list.add_head(1));
    assert_eq!(list.delete_at(2), None);
    assert_eq!(list.delete_key(9), None);
}
