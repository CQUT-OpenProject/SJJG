use ex_1::{RecType, sample_data, sort_records};

fn keys(r: &[RecType]) -> Vec<i32> {
    r.iter().map(|data| data.key).collect()
}

#[test]
fn quicksort_sorts_sample_data() {
    let mut r = sample_data();

    sort_records(&mut r);

    assert_eq!(keys(&r), vec![13, 27, 38, 49, 65, 76, 97]);
}

#[test]
fn quicksort_handles_empty_and_one_data() {
    let mut empty: Vec<RecType> = Vec::new();
    sort_records(&mut empty);
    assert!(empty.is_empty());

    let mut one = vec![RecType { no: 1, key: 8 }];
    sort_records(&mut one);
    assert_eq!(keys(&one), vec![8]);
}
