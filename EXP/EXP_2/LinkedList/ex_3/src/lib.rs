#[derive(Debug, Clone)]
struct Node {
    key: i32,
    next: usize,
    active: bool,
}

#[derive(Debug, Clone, Default)]
pub struct CircularList {
    nodes: Vec<Node>,
    head: Option<usize>,
    len: usize,
}

impl CircularList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn output(&self) -> Vec<i32> {
        self.circular_output(self.len)
    }

    pub fn circular_output(&self, count: usize) -> Vec<i32> {
        let mut result = Vec::new();
        let Some(mut index) = self.head else {
            return result;
        };

        for _ in 0..count {
            result.push(self.nodes[index].key);
            index = self.nodes[index].next;
        }

        result
    }

    pub fn get(&self, pos: usize) -> Option<i32> {
        let index = self.index_at(pos)?;
        Some(self.nodes[index].key)
    }

    pub fn delete_at(&mut self, pos: usize) -> Option<i32> {
        if pos == 0 || pos > self.len {
            return None;
        }

        let target = self.index_at(pos)?;
        let key = self.nodes[target].key;

        if self.len == 1 {
            self.nodes[target].active = false;
            self.head = None;
            self.len = 0;
            return Some(key);
        }

        let prev = if pos == 1 {
            self.index_at(self.len)?
        } else {
            self.index_at(pos - 1)?
        };

        self.nodes[prev].next = self.nodes[target].next;
        self.nodes[target].active = false;
        if self.head == Some(target) {
            self.head = Some(self.nodes[target].next);
        }
        self.len -= 1;
        Some(key)
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

        let new_index = self.nodes.len();
        if self.len == 0 {
            self.nodes.push(Node {
                key,
                next: new_index,
                active: true,
            });
            self.head = Some(new_index);
            self.len = 1;
            return true;
        }

        let next = if pos == self.len + 1 {
            self.head.unwrap()
        } else {
            self.index_at(pos).unwrap()
        };
        let prev = if pos == 1 {
            self.index_at(self.len).unwrap()
        } else {
            self.index_at(pos - 1).unwrap()
        };

        self.nodes.push(Node {
            key,
            next,
            active: true,
        });
        self.nodes[prev].next = new_index;
        if pos == 1 {
            self.head = Some(new_index);
        }
        self.len += 1;
        true
    }

    pub fn search(&self, key: i32) -> Option<usize> {
        let mut index = self.head?;
        for pos in 1..=self.len {
            let node = &self.nodes[index];
            if node.active && node.key == key {
                return Some(pos);
            }
            index = node.next;
        }

        None
    }

    fn index_at(&self, pos: usize) -> Option<usize> {
        if pos == 0 || pos > self.len {
            return None;
        }

        let mut index = self.head?;
        for _ in 1..pos {
            index = self.nodes[index].next;
        }

        Some(index)
    }
}
