pub fn bin_search(a: &[i32], x: i32) -> i32 {
    if a.is_empty() {
        return -1;
    }

    let mut low = 0usize;
    let mut high = a.len() - 1;

    // 有序表适合用折半查找，比较次数会少很多。
    while low <= high {
        let mid = (low + high) / 2;

        if a[mid] == x {
            return mid as i32;
        }

        if a[mid] < x {
            low = mid + 1;
        } else {
            if mid == 0 {
                break;
            }
            high = mid - 1;
        }
    }

    -1
}

pub fn sample_data() -> [i32; 10] {
    [3, 8, 12, 15, 18, 21, 25, 30, 36, 42]
}
