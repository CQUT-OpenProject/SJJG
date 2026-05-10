#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub data: i32,
    pub next: Option<Box<Node>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkQueue {
    head: Option<Box<Node>>,
    len: usize,
}

impl LinkQueue {
    pub fn new() -> Self {
        Self { head: None, len: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn is_full(&self) -> bool {
        false
    }

    pub fn enqueue(&mut self, value: i32) {
        let node = Box::new(Node {
            data: value,
            next: None,
        });

        match self.head.as_mut() {
            None => {
                self.head = Some(node);
            }
            Some(curr) => {
                let mut tail = curr;
                // 一直走到链表最后一个结点，再把新结点接到队尾。
                while let Some(ref mut next) = tail.next {
                    tail = next;
                }
                tail.next = Some(node);
            }
        }

        self.len += 1;
    }

    pub fn dequeue(&mut self) -> Option<i32> {
        let mut node = self.head.take()?;
        // 队列出队发生在队头，删掉头结点即可。
        self.head = node.next.take();
        self.len -= 1;
        Some(node.data)
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn output(&self) -> Vec<i32> {
        let mut result = Vec::new();
        let mut curr = self.head.as_ref();

        while let Some(node) = curr {
            result.push(node.data);
            curr = node.next.as_ref();
        }

        result
    }
}

impl Default for LinkQueue {
    fn default() -> Self {
        Self::new()
    }
}
