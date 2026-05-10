#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeqCharStack {
    data: Vec<char>,
}

impl SeqCharStack {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn push(&mut self, ch: char) {
        self.data.push(ch);
    }

    pub fn pop(&mut self) -> Option<char> {
        self.data.pop()
    }
}

impl Default for SeqCharStack {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkNode {
    pub data: char,
    pub next: Option<Box<LinkNode>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkCharStack {
    head: Option<Box<LinkNode>>,
}

impl LinkCharStack {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, ch: char) {
        // 链栈仍然采用头插法，链表头就是当前栈顶。
        let node = Box::new(LinkNode {
            data: ch,
            next: self.head.take(),
        });
        self.head = Some(node);
    }

    pub fn pop(&mut self) -> Option<char> {
        let mut node = self.head.take()?;
        self.head = node.next.take();
        Some(node.data)
    }
}

impl Default for LinkCharStack {
    fn default() -> Self {
        Self::new()
    }
}

fn normalize_text(text: &str) -> Vec<char> {
    // 只保留字母和数字，并统一转成小写，便于后面比较。
    text.chars()
        .filter(|ch| ch.is_alphanumeric())
        .flat_map(|ch| ch.to_lowercase())
        .collect()
}

pub fn is_palindrome_seq(text: &str) -> bool {
    let data = normalize_text(text);
    let mut stack = SeqCharStack::new();

    for &ch in &data {
        stack.push(ch);
    }

    // 再按原顺序比较，弹栈得到的正好是反向字符序列。
    for &ch in &data {
        if stack.pop() != Some(ch) {
            return false;
        }
    }

    true
}

pub fn is_palindrome_link(text: &str) -> bool {
    let data = normalize_text(text);
    let mut stack = LinkCharStack::new();

    for &ch in &data {
        stack.push(ch);
    }

    // 链栈版本与顺序栈版本流程相同，只是存储方式不同。
    for &ch in &data {
        if stack.pop() != Some(ch) {
            return false;
        }
    }

    true
}
