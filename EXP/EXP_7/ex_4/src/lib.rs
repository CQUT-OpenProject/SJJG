pub fn bin_insert_pos(a: &[i32], x: i32) -> usize {
    let mut low = 0usize;
    let mut high = a.len();

    // 找到第一个不小于 x 的位置，后面的元素统一后移。
    while low < high {
        let mid = (low + high) / 2;
        if a[mid] < x {
            low = mid + 1;
        } else {
            high = mid;
        }
    }

    low
}

pub fn insert_ordered(a: &mut Vec<i32>, x: i32) -> usize {
    let pos = bin_insert_pos(a, x);
    a.push(0);

    let mut i = a.len() - 1;
    while i > pos {
        a[i] = a[i - 1];
        i -= 1;
    }

    a[pos] = x;
    pos
}

pub fn sample_data() -> Vec<i32> {
    vec![3, 8, 12, 15, 18, 21, 25, 30, 36, 42]
}
