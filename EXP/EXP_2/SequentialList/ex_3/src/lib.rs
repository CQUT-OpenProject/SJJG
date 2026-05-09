#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MinMax {
    pub min: i32,
    pub max: i32,
    pub comparisons: usize,
}

pub fn find_min_max(a: &[i32]) -> Option<MinMax> {
    if a.is_empty() {
        return None;
    }

    let mut comparisons = 0;
    let (mut min, mut max, mut index) = if a.len() % 2 == 0 {
        comparisons += 1;
        if a[0] < a[1] {
            (a[0], a[1], 2)
        } else {
            (a[1], a[0], 2)
        }
    } else {
        (a[0], a[0], 1)
    };

    while index + 1 < a.len() {
        let (small, large) = {
            comparisons += 1;
            if a[index] < a[index + 1] {
                (a[index], a[index + 1])
            } else {
                (a[index + 1], a[index])
            }
        };

        comparisons += 1;
        if small < min {
            min = small;
        }

        comparisons += 1;
        if large > max {
            max = large;
        }

        index += 2;
    }

    Some(MinMax {
        min,
        max,
        comparisons,
    })
}
