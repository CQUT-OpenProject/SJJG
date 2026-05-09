pub fn odd_before_even(data: &mut [i32]) {
    if data.len() < 2 {
        return;
    }

    let mut i = 0;
    let mut j = data.len() - 1;

    while i < j {
        while i < j && data[i] % 2 != 0 {
            i += 1;
        }

        while i < j && data[j] % 2 == 0 {
            j -= 1;
        }

        if i < j {
            data.swap(i, j);
            i += 1;
            j -= 1;
        }
    }
}
