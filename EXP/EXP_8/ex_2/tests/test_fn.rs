use ex_2::{bubble_sort, insert_sort, quick_sort, sample_data, select_sort};

fn sorted_data() -> Vec<i32> {
    let mut data = sample_data().to_vec();
    data.sort();
    data
}

#[test]
fn insert_sort_sorts_data() {
    let mut a = sample_data();
    insert_sort(&mut a);
    assert_eq!(a.to_vec(), sorted_data());
}

#[test]
fn select_sort_sorts_data() {
    let mut a = sample_data();
    select_sort(&mut a);
    assert_eq!(a.to_vec(), sorted_data());
}

#[test]
fn bubble_sort_sorts_data() {
    let mut a = sample_data();
    bubble_sort(&mut a);
    assert_eq!(a.to_vec(), sorted_data());
}

#[test]
fn quick_sort_sorts_data() {
    let mut a = sample_data();
    quick_sort(&mut a);
    assert_eq!(a.to_vec(), sorted_data());
}
