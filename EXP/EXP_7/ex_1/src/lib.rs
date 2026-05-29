pub fn search(a: &[i32], x: i32) -> i32 {
    let mut i = 0;

    // 无序表只能从前往后逐个比较，找到就返回下标。
    while i < a.len() && a[i] != x {
        i += 1;
    }

    if i >= a.len() { -1 } else { i as i32 }
}

pub fn sample_data() -> [i32; 10] {
    [2, 5, 56, 10, 12, 15, 8, 19, 25, 32]
}
