use seq_ex_2::split_positive_negative;

#[test]
fn split_keeps_positive_and_negative_order() {
    let a = [3, -1, 8, -6, 5, -2];
    let (b, c) = split_positive_negative(&a);

    assert_eq!(b, vec![3, 8, 5]);
    assert_eq!(c, vec![-1, -6, -2]);
}

#[test]
fn split_handles_single_side_lists() {
    assert_eq!(split_positive_negative(&[1, 2, 3]), (vec![1, 2, 3], vec![]));
    assert_eq!(split_positive_negative(&[-1, -2]), (vec![], vec![-1, -2]));
}
