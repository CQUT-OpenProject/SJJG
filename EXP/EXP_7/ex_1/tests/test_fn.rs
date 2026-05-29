use ex_1::{sample_data, search};

#[test]
fn search_finds_existing_data() {
    let a = sample_data();

    assert_eq!(search(&a, 2), 0);
    assert_eq!(search(&a, 15), 5);
    assert_eq!(search(&a, 32), 9);
}

#[test]
fn search_returns_minus_one_when_data_missing() {
    let a = sample_data();

    assert_eq!(search(&a, 100), -1);
    assert_eq!(search(&a, -3), -1);
}
