use ex_4::{heap_sort, sample_data};

#[test]
fn heap_sort_sorts_data() {
    let mut a = sample_data();
    let mut sorted = a.to_vec();
    sorted.sort();

    heap_sort(&mut a);

    assert_eq!(a.to_vec(), sorted);
}

#[test]
fn heap_sort_handles_small_data() {
    let mut empty: Vec<i32> = Vec::new();
    heap_sort(&mut empty);
    assert!(empty.is_empty());

    let mut one = vec![8];
    heap_sort(&mut one);
    assert_eq!(one, vec![8]);
}
