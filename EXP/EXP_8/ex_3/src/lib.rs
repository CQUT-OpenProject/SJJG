pub fn shell_sort(a: &mut [i32]) {
    let mut gap = a.len() / 2;

    while gap > 0 {
        let mut i = gap;

        while i < a.len() {
            let temp = a[i];
            let mut j = i;

            // 间隔为 gap 的插入排序。
            while j >= gap && a[j - gap] > temp {
                a[j] = a[j - gap];
                j -= gap;
            }
            a[j] = temp;

            i += 1;
        }

        gap /= 2;
    }
}

fn merge(a: &mut [i32], temp: &mut [i32], low: usize, mid: usize, high: usize) {
    let mut i = low;
    let mut j = mid + 1;
    let mut k = low;

    while i <= mid && j <= high {
        if a[i] <= a[j] {
            temp[k] = a[i];
            i += 1;
        } else {
            temp[k] = a[j];
            j += 1;
        }
        k += 1;
    }

    while i <= mid {
        temp[k] = a[i];
        i += 1;
        k += 1;
    }

    while j <= high {
        temp[k] = a[j];
        j += 1;
        k += 1;
    }

    k = low;
    while k <= high {
        a[k] = temp[k];
        k += 1;
    }
}

fn merge_sort_part(a: &mut [i32], temp: &mut [i32], low: usize, high: usize) {
    if low < high {
        let mid = (low + high) / 2;
        merge_sort_part(a, temp, low, mid);
        merge_sort_part(a, temp, mid + 1, high);
        merge(a, temp, low, mid, high);
    }
}

pub fn merge_sort(a: &mut [i32]) {
    if !a.is_empty() {
        let mut temp = a.to_vec();
        merge_sort_part(a, &mut temp, 0, a.len() - 1);
    }
}

pub fn sample_data() -> [i32; 10] {
    [49, 38, 65, 97, 76, 13, 27, 50, 4, 88]
}
