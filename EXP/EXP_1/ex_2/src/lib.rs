pub fn exchange(a: &mut [i32]) -> bool {
    if a.len() < 2 {
        return false;
    }

    let mut min_idx = 0;
    let mut max_idx = 0;

    for i in 1..a.len() {
        if a[i] < a[min_idx] {
            min_idx = i;
        }
        if a[i] > a[max_idx] {
            max_idx = i;
        }
    }

    a.swap(min_idx, max_idx);

    true
}
