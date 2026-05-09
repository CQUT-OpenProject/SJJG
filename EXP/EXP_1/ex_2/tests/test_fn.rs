use ex_2::exchange;

#[test]
fn exchange_swaps_min_and_max_basic_case() {
    let mut data = [10, 5, 8, 2, 9];
    let changed = exchange(&mut data);

    assert!(changed);
    assert_eq!(data, [2, 5, 8, 10, 9]);
}

#[test]
fn exchange_swaps_when_extremes_at_ends() {
    let mut data = [1, 3, 2, 9];
    let changed = exchange(&mut data);

    assert!(changed);
    assert_eq!(data, [9, 3, 2, 1]);
}

#[test]
fn exchange_returns_false_for_empty_slice() {
    let mut data: [i32; 0] = [];
    let changed = exchange(&mut data);

    assert!(!changed);
    assert_eq!(data, []);
}

#[test]
fn exchange_returns_false_for_single_element() {
    let mut data = [42];
    let changed = exchange(&mut data);

    assert!(!changed);
    assert_eq!(data, [42]);
}

#[test]
fn exchange_uses_first_occurrence_for_duplicate_min_max() {
    let mut data = [4, 1, 3, 1, 9, 9, 2];
    let changed = exchange(&mut data);

    assert!(changed);
    assert_eq!(data, [4, 9, 3, 1, 1, 9, 2]);
}

#[test]
fn exchange_keeps_all_equal_values_unchanged() {
    let mut data = [7, 7, 7, 7];
    let changed = exchange(&mut data);

    assert!(changed);
    assert_eq!(data, [7, 7, 7, 7]);
}
