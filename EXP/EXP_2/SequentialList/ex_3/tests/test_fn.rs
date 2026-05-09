use seq_ex_3::{MinMax, find_min_max};

#[test]
fn finds_min_and_max_in_odd_length_list() {
    let result = find_min_max(&[7, 3, 9, -2, 5]).unwrap();

    assert_eq!(
        result,
        MinMax {
            min: -2,
            max: 9,
            comparisons: 6
        }
    );
    assert!(result.comparisons <= 3 * 5 / 2);
}

#[test]
fn finds_min_and_max_in_even_length_list() {
    let result = find_min_max(&[10, -4, 6, 18, 2, 0]).unwrap();

    assert_eq!(result.min, -4);
    assert_eq!(result.max, 18);
    assert_eq!(result.comparisons, 7);
    assert!(result.comparisons <= 3 * 6 / 2);
}

#[test]
fn empty_list_has_no_min_or_max() {
    assert_eq!(find_min_max(&[]), None);
}
