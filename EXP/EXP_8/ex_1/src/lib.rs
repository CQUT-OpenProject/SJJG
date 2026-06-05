#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecType {
    pub key: i32,
    pub no: i32,
}

pub fn partition(r: &mut [RecType], mut i: usize, mut j: usize) -> usize {
    let pivot = r[i];

    while i < j {
        while i < j && r[j].key >= pivot.key {
            j -= 1;
        }
        if i < j {
            r[i] = r[j];
            i += 1;
        }

        while i < j && r[i].key <= pivot.key {
            i += 1;
        }
        if i < j {
            r[j] = r[i];
            j -= 1;
        }
    }

    r[i] = pivot;
    i
}

pub fn quicksort(r: &mut [RecType], low: usize, high: usize) {
    if low < high {
        let pivotpos = partition(r, low, high);

        if pivotpos > 0 {
            quicksort(r, low, pivotpos - 1);
        }
        quicksort(r, pivotpos + 1, high);
    }
}

pub fn sort_records(r: &mut [RecType]) {
    if !r.is_empty() {
        quicksort(r, 0, r.len() - 1);
    }
}

pub fn sample_data() -> Vec<RecType> {
    vec![
        RecType { no: 1, key: 49 },
        RecType { no: 2, key: 38 },
        RecType { no: 3, key: 65 },
        RecType { no: 4, key: 97 },
        RecType { no: 5, key: 76 },
        RecType { no: 6, key: 13 },
        RecType { no: 7, key: 27 },
    ]
}
