pub fn split_positive_negative(a: &[i32]) -> (Vec<i32>, Vec<i32>) {
    let mut b = Vec::new();
    let mut c = Vec::new();

    for &item in a {
        if item > 0 {
            b.push(item);
        } else if item < 0 {
            c.push(item);
        }
    }

    (b, c)
}
