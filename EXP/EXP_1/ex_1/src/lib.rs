pub fn fun(arr: &[i32], odd: &mut i32, even: &mut i32) {
    *odd = 0;
    *even = 0;

    for &num in arr {
        if num % 2 == 0 {
            *even += num;
        } else {
            *odd += num;
        }
    }
}
