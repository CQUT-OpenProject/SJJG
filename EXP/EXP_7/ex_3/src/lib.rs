#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndexNode {
    pub start: usize,
    pub max_key: i32,
}

pub fn sample_data() -> [i32; 18] {
    [
        22, 12, 13, 8, 9, 20, 33, 42, 44, 38, 24, 48, 60, 58, 74, 49, 86, 53,
    ]
}

pub fn build_index(data: &[i32], block_size: usize) -> Vec<IndexNode> {
    let mut index = Vec::new();
    let mut start = 0;

    while start < data.len() {
        let mut end = start + block_size;
        if end > data.len() {
            end = data.len();
        }

        let mut max_key = data[start];
        let mut i = start + 1;
        while i < end {
            if data[i] > max_key {
                max_key = data[i];
            }
            i += 1;
        }

        index.push(IndexNode { start, max_key });
        start = end;
    }

    index
}

pub fn block_search(data: &[i32], index: &[IndexNode], block_size: usize, key: i32) -> i32 {
    let mut low = 0usize;
    let mut high = index.len();

    // 先在索引表中折半查找，找到第一个最大关键字不小于 key 的块。
    while low < high {
        let mid = (low + high) / 2;
        if index[mid].max_key < key {
            low = mid + 1;
        } else {
            high = mid;
        }
    }

    if low >= index.len() {
        return -1;
    }

    let mut i = index[low].start;
    let mut end = i + block_size;
    if end > data.len() {
        end = data.len();
    }

    while i < end {
        if data[i] == key {
            return i as i32;
        }
        i += 1;
    }

    -1
}

pub fn sample_index() -> Vec<IndexNode> {
    let data = sample_data();
    build_index(&data, 6)
}
