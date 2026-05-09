#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeqList {
    data: Vec<i32>,
    max_len: usize,
}

impl SeqList {
    pub fn new(max_len: usize) -> Self {
        Self {
            data: Vec::new(),
            max_len: max_len.min(1024),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn output(&self) -> Vec<i32> {
        self.data.clone()
    }

    pub fn get(&self, pos: usize) -> Option<i32> {
        if pos == 0 {
            return None;
        }

        self.data.get(pos - 1).copied()
    }

    pub fn delete_at(&mut self, pos: usize) -> Option<i32> {
        if pos == 0 || pos > self.data.len() {
            return None;
        }

        Some(self.data.remove(pos - 1))
    }

    pub fn delete_key(&mut self, key: i32) -> Option<i32> {
        let pos = self.search(key)?;
        self.delete_at(pos)
    }

    pub fn add_head(&mut self, key: i32) -> bool {
        self.insert(1, key)
    }

    pub fn add_tail(&mut self, key: i32) -> bool {
        self.insert(self.data.len() + 1, key)
    }

    pub fn insert(&mut self, pos: usize, key: i32) -> bool {
        if pos == 0 || pos > self.data.len() + 1 {
            return false;
        }
        if self.data.len() >= self.max_len || self.search(key).is_some() {
            return false;
        }

        self.data.insert(pos - 1, key);
        true
    }

    pub fn search(&self, key: i32) -> Option<usize> {
        self.data
            .iter()
            .position(|&item| item == key)
            .map(|idx| idx + 1)
    }
}
