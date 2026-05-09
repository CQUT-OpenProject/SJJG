#[derive(Debug, Clone)]
struct Node {
    key: char,
    next: Option<Box<Node>>,
}

#[derive(Debug, Clone, Default)]
pub struct SinglyList {
    head: Option<Box<Node>>,
    len: usize,
}

impl SinglyList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn output(&self) -> Vec<char> {
        let mut result = Vec::new();
        let mut current = self.head.as_ref();

        while let Some(node) = current {
            result.push(node.key);
            current = node.next.as_ref();
        }

        result
    }

    pub fn get(&self, pos: usize) -> Option<char> {
        if pos == 0 || pos > self.len {
            return None;
        }

        let mut current = self.head.as_ref();
        for _ in 1..pos {
            current = current?.next.as_ref();
        }

        current.map(|node| node.key)
    }

    pub fn delete_at(&mut self, pos: usize) -> Option<char> {
        if pos == 0 || pos > self.len {
            return None;
        }

        let mut link = &mut self.head;
        for _ in 1..pos {
            link = &mut link.as_mut()?.next;
        }

        let mut target = link.take()?;
        *link = target.next.take();
        self.len -= 1;
        Some(target.key)
    }

    pub fn delete_key(&mut self, key: char) -> Option<char> {
        let pos = self.search(key)?;
        self.delete_at(pos)
    }

    pub fn add_head(&mut self, key: char) -> bool {
        self.insert(1, key)
    }

    pub fn add_tail(&mut self, key: char) -> bool {
        self.insert(self.len + 1, key)
    }

    pub fn insert(&mut self, pos: usize, key: char) -> bool {
        if pos == 0 || pos > self.len + 1 || self.search(key).is_some() {
            return false;
        }

        let mut link = &mut self.head;
        for _ in 1..pos {
            link = match link.as_mut() {
                Some(node) => &mut node.next,
                None => return false,
            };
        }

        let node = Box::new(Node {
            key,
            next: link.take(),
        });
        *link = Some(node);
        self.len += 1;
        true
    }

    pub fn search(&self, key: char) -> Option<usize> {
        let mut current = self.head.as_ref();
        let mut pos = 1;

        while let Some(node) = current {
            if node.key == key {
                return Some(pos);
            }
            current = node.next.as_ref();
            pos += 1;
        }

        None
    }
}
