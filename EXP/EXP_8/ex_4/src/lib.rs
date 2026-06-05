fn sift(a: &mut [i32], mut low: usize, high: usize) {
    let temp = a[low];
    let mut i = low * 2 + 1;

    while i <= high {
        if i < high && a[i] < a[i + 1] {
            i += 1;
        }

        if temp < a[i] {
            a[low] = a[i];
            low = i;
            i = low * 2 + 1;
        } else {
            break;
        }
    }

    a[low] = temp;
}

pub fn heap_sort(a: &mut [i32]) {
    if a.len() < 2 {
        return;
    }

    let mut i = a.len() / 2;
    while i > 0 {
        i -= 1;
        sift(a, i, a.len() - 1);
    }

    let mut high = a.len() - 1;
    while high > 0 {
        a.swap(0, high);
        high -= 1;
        sift(a, 0, high);
    }
}

pub fn sample_data() -> [i32; 10] {
    [49, 38, 65, 97, 76, 13, 27, 50, 4, 88]
}
