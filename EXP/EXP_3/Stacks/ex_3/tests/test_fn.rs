use stack_ex_3::{from_slice, reverse_list, to_vec};

#[test]
fn reverse_linked_list_with_stack() {
    let head = from_slice(&[1, 2, 3, 4, 5]);
    let reversed = reverse_list(head);

    assert_eq!(to_vec(&reversed), vec![5, 4, 3, 2, 1]);
}

#[test]
fn reverse_empty_or_single_list() {
    let empty = reverse_list(None);
    assert_eq!(to_vec(&empty), Vec::<i32>::new());

    let single = reverse_list(from_slice(&[7]));
    assert_eq!(to_vec(&single), vec![7]);
}
