use seq_ex_1::SeqList;

#[test]
fn sequential_list_supports_table_10_7_operations() {
    let mut list = SeqList::new(5);

    assert!(list.is_empty());
    assert_eq!(list.output(), Vec::<i32>::new());

    assert!(list.add_tail(20));
    assert!(list.add_head(10));
    assert!(list.insert(2, 15));
    assert!(list.add_tail(30));

    assert_eq!(list.output(), vec![10, 15, 20, 30]);
    assert_eq!(list.get(1), Some(10));
    assert_eq!(list.get(4), Some(30));
    assert_eq!(list.search(20), Some(3));
    assert_eq!(list.search(99), None);

    assert_eq!(list.delete_at(2), Some(15));
    assert_eq!(list.delete_key(30), Some(30));
    assert_eq!(list.output(), vec![10, 20]);
}

#[test]
fn sequential_list_rejects_invalid_or_duplicate_updates() {
    let mut list = SeqList::new(2);

    assert!(!list.insert(0, 1));
    assert!(!list.delete_at(1).is_some());

    assert!(list.add_tail(1));
    assert!(!list.add_tail(1));
    assert!(list.add_tail(2));
    assert!(!list.add_head(3));
    assert!(!list.insert(4, 3));
    assert_eq!(list.output(), vec![1, 2]);
}
