#[derive(Debug, Clone)]
struct Node {
    key: Option<i32>,
    prev: usize,
    next: usize,
    active: bool,
}

#[derive(Debug, Clone)]
pub struct DoublyList {
    nodes: Vec<Node>,
    len: usize,
}

impl Default for DoublyList {
    fn default() -> Self {
        Self {
            nodes: vec![Node {
                key: None,
                prev: 0,
                next: 0,
                active: true,
            }],
            len: 0,
        }
    }
}

impl DoublyList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn output(&self) -> Vec<i32> {
        let mut result = Vec::new();
        let mut index = self.nodes[0].next;

        while index != 0 {
            result.push(self.nodes[index].key.unwrap());
            index = self.nodes[index].next;
        }

        result
    }

    pub fn reverse_output(&self) -> Vec<i32> {
        let mut result = Vec::new();
        let mut index = self.nodes[0].prev;

        while index != 0 {
            result.push(self.nodes[index].key.unwrap());
            index = self.nodes[index].prev;
        }

        result
    }

    pub fn get(&self, pos: usize) -> Option<i32> {
        let index = self.index_at(pos)?;
        self.nodes[index].key
    }

    pub fn delete_at(&mut self, pos: usize) -> Option<i32> {
        let index = self.index_at(pos)?;
        self.unlink(index)
    }

    pub fn delete_key(&mut self, key: i32) -> Option<i32> {
        let pos = self.search(key)?;
        self.delete_at(pos)
    }

    pub fn add_head(&mut self, key: i32) -> bool {
        self.insert(1, key)
    }

    pub fn add_tail(&mut self, key: i32) -> bool {
        self.insert(self.len + 1, key)
    }

    pub fn insert(&mut self, pos: usize, key: i32) -> bool {
        if pos == 0 || pos > self.len + 1 || self.search(key).is_some() {
            return false;
        }

        let next = if pos == self.len + 1 {
            0
        } else {
            self.index_at(pos).unwrap()
        };
        let prev = self.nodes[next].prev;
        let new_index = self.nodes.len();

        self.nodes.push(Node {
            key: Some(key),
            prev,
            next,
            active: true,
        });
        self.nodes[prev].next = new_index;
        self.nodes[next].prev = new_index;
        self.len += 1;
        true
    }

    pub fn search(&self, key: i32) -> Option<usize> {
        let mut index = self.nodes[0].next;
        let mut pos = 1;

        while index != 0 {
            if self.nodes[index].active && self.nodes[index].key == Some(key) {
                return Some(pos);
            }
            index = self.nodes[index].next;
            pos += 1;
        }

        None
    }

    fn index_at(&self, pos: usize) -> Option<usize> {
        if pos == 0 || pos > self.len {
            return None;
        }

        let mut index = self.nodes[0].next;
        for _ in 1..pos {
            index = self.nodes[index].next;
        }

        Some(index)
    }

    fn unlink(&mut self, index: usize) -> Option<i32> {
        if index == 0 || !self.nodes[index].active {
            return None;
        }

        let prev = self.nodes[index].prev;
        let next = self.nodes[index].next;
        self.nodes[prev].next = next;
        self.nodes[next].prev = prev;
        self.nodes[index].active = false;
        self.len -= 1;
        self.nodes[index].key
    }
}
