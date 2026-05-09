pub fn move_non_zero_front(data: &mut [i32]) {
    let mut i = 0;

    for j in 0..data.len() {
        if data[j] != 0 {
            if i != j {
                data.swap(i, j);
            }
            i += 1;
        }
    }
}
