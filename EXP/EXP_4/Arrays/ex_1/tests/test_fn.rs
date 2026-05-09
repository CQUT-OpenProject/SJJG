use array_ex_1::move_non_zero_front;

#[test]
fn moves_non_zero_items_to_front() {
    let mut data = vec![0, 3, 0, 8, 0, 2, 5];

    move_non_zero_front(&mut data);

    assert_eq!(data, vec![3, 8, 2, 5, 0, 0, 0]);
}

#[test]
fn handles_all_zero_and_no_zero_lists() {
    let mut all_zero = vec![0, 0, 0];
    let mut no_zero = vec![1, 2, 3];

    move_non_zero_front(&mut all_zero);
    move_non_zero_front(&mut no_zero);

    assert_eq!(all_zero, vec![0, 0, 0]);
    assert_eq!(no_zero, vec![1, 2, 3]);
}
