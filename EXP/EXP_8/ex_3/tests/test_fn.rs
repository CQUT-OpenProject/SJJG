use ex_3::{merge_sort, sample_data, shell_sort};

fn sorted_data() -> Vec<i32> {
    let mut data = sample_data().to_vec();
    data.sort();
    data
}

#[test]
fn shell_sort_sorts_data() {
    let mut a = sample_data();
    shell_sort(&mut a);
    assert_eq!(a.to_vec(), sorted_data());
}

#[test]
fn merge_sort_sorts_data() {
    let mut a = sample_data();
    merge_sort(&mut a);
    assert_eq!(a.to_vec(), sorted_data());
}
