#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkNode {
    pub info: i32,
    pub next: Option<Box<LinkNode>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkStack {
    head: Option<Box<LinkNode>>,
    len: usize,
}

impl LinkStack {
    pub fn new() -> Self {
        Self { head: None, len: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn is_full(&self) -> bool {
        false
    }

    pub fn push(&mut self, value: i32) {
        // 链栈把链表头当作栈顶，新结点直接插到最前面。
        let node = Box::new(LinkNode {
            info: value,
            next: self.head.take(),
        });
        self.head = Some(node);
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<i32> {
        let mut node = self.head.take()?;
        // 取下头结点后，新的栈顶就是原来第二个结点。
        self.head = node.next.take();
        self.len -= 1;
        Some(node.info)
    }

    pub fn top(&self) -> Option<i32> {
        self.head.as_ref().map(|node| node.info)
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn output(&self) -> Vec<i32> {
        let mut data = Vec::new();
        let mut curr = self.head.as_ref();

        // 从栈顶往栈底依次读取链栈中的元素。
        while let Some(node) = curr {
            data.push(node.info);
            curr = node.next.as_ref();
        }

        data
    }
}

impl Default for LinkStack {
    fn default() -> Self {
        Self::new()
    }
}
