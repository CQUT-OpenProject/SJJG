use ex_2::{bin_search, sample_data};

#[test]
fn bin_search_finds_existing_data() {
    let a = sample_data();

    assert_eq!(bin_search(&a, 3), 0);
    assert_eq!(bin_search(&a, 21), 5);
    assert_eq!(bin_search(&a, 42), 9);
}

#[test]
fn bin_search_returns_minus_one_when_data_missing() {
    let a = sample_data();

    assert_eq!(bin_search(&a, 7), -1);
    assert_eq!(bin_search(&a, 100), -1);
}

#[test]
fn bin_search_handles_empty_table() {
    let a: [i32; 0] = [];

    assert_eq!(bin_search(&a, 1), -1);
}
