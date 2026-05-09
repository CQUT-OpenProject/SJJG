use link_ex_3::CircularList;

#[test]
fn circular_list_supports_table_10_7_operations() {
    let mut list = CircularList::new();

    assert!(list.add_tail(2));
    assert!(list.add_head(1));
    assert!(list.insert(3, 4));
    assert!(list.insert(3, 3));

    assert_eq!(list.output(), vec![1, 2, 3, 4]);
    assert_eq!(list.circular_output(6), vec![1, 2, 3, 4, 1, 2]);
    assert_eq!(list.get(4), Some(4));
    assert_eq!(list.search(3), Some(3));
    assert_eq!(list.delete_key(1), Some(1));
    assert_eq!(list.delete_at(3), Some(4));
    assert_eq!(list.output(), vec![2, 3]);
    assert_eq!(list.circular_output(5), vec![2, 3, 2, 3, 2]);
}

#[test]
fn circular_list_handles_empty_and_singleton_cases() {
    let mut list = CircularList::new();

    assert_eq!(list.delete_at(1), None);
    assert_eq!(list.circular_output(3), Vec::<i32>::new());
    assert!(list.add_head(9));
    assert_eq!(list.circular_output(4), vec![9, 9, 9, 9]);
    assert_eq!(list.delete_key(9), Some(9));
    assert_eq!(list.output(), Vec::<i32>::new());
}
