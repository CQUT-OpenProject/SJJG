#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeqStack {
    data: Vec<char>,
}

impl SeqStack {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn push(&mut self, ch: char) {
        self.data.push(ch);
    }

    pub fn pop(&mut self) -> Option<char> {
        self.data.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl Default for SeqStack {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkNode {
    ch: char,
    next: Option<Box<LinkNode>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkStack {
    head: Option<Box<LinkNode>>,
}

impl LinkStack {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, ch: char) {
        // 扫描到左花括号时，把它压到链栈栈顶。
        let node = Box::new(LinkNode {
            ch,
            next: self.head.take(),
        });
        self.head = Some(node);
    }

    pub fn pop(&mut self) -> Option<char> {
        let mut node = self.head.take()?;
        self.head = node.next.take();
        Some(node.ch)
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

impl Default for LinkStack {
    fn default() -> Self {
        Self::new()
    }
}

pub fn brackets_match_seq(text: &str) -> bool {
    let mut stack = SeqStack::new();

    for ch in text.chars() {
        match ch {
            '{' => stack.push(ch),
            '}' => {
                // 遇到右花括号时，如果没有可弹出的左花括号就说明不匹配。
                if stack.pop().is_none() {
                    return false;
                }
            }
            _ => {}
        }
    }

    stack.is_empty()
}

pub fn brackets_match_link(text: &str) -> bool {
    let mut stack = LinkStack::new();

    for ch in text.chars() {
        match ch {
            '{' => stack.push(ch),
            '}' => {
                if stack.pop().is_none() {
                    return false;
                }
            }
            _ => {}
        }
    }

    // 全部字符读完后栈为空，说明每个左花括号都找到了对应的右花括号。
    stack.is_empty()
}
