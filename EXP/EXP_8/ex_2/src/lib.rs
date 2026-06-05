pub fn insert_sort(a: &mut [i32]) {
    let mut i = 1;

    while i < a.len() {
        let temp = a[i];
        let mut j = i;

        // 从已经排好序的一段里，给 temp 找到插入位置。
        while j > 0 && a[j - 1] > temp {
            a[j] = a[j - 1];
            j -= 1;
        }
        a[j] = temp;

        i += 1;
    }
}

pub fn select_sort(a: &mut [i32]) {
    let mut i = 0;

    while i < a.len() {
        let mut min = i;
        let mut j = i + 1;

        while j < a.len() {
            if a[j] < a[min] {
                min = j;
            }
            j += 1;
        }

        if min != i {
            a.swap(i, min);
        }
        i += 1;
    }
}

pub fn bubble_sort(a: &mut [i32]) {
    if a.len() < 2 {
        return;
    }

    let mut end = a.len() - 1;

    while end > 0 {
        let mut exchange = false;
        let mut j = 0;

        while j < end {
            if a[j] > a[j + 1] {
                a.swap(j, j + 1);
                exchange = true;
            }
            j += 1;
        }

        if !exchange {
            break;
        }
        end -= 1;
    }
}

fn partition(a: &mut [i32], mut low: usize, mut high: usize) -> usize {
    let pivot = a[low];

    while low < high {
        while low < high && a[high] >= pivot {
            high -= 1;
        }
        if low < high {
            a[low] = a[high];
            low += 1;
        }

        while low < high && a[low] <= pivot {
            low += 1;
        }
        if low < high {
            a[high] = a[low];
            high -= 1;
        }
    }

    a[low] = pivot;
    low
}

fn quicksort_part(a: &mut [i32], low: usize, high: usize) {
    if low < high {
        let pivotpos = partition(a, low, high);

        if pivotpos > 0 {
            quicksort_part(a, low, pivotpos - 1);
        }
        quicksort_part(a, pivotpos + 1, high);
    }
}

pub fn quick_sort(a: &mut [i32]) {
    if !a.is_empty() {
        quicksort_part(a, 0, a.len() - 1);
    }
}

pub fn sample_data() -> [i32; 10] {
    [49, 38, 65, 97, 76, 13, 27, 50, 4, 88]
}
