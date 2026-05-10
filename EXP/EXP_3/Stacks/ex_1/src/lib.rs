pub const MAX: usize = 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeqStack {
    data: [i32; MAX],
    top: usize,
}

impl SeqStack {
    pub fn new() -> Self {
        Self {
            data: [0; MAX],
            top: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.top == 0
    }

    pub fn is_full(&self) -> bool {
        self.top >= MAX
    }

    pub fn push(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }

        // top 指向下一个可用位置，所以先写入再后移。
        self.data[self.top] = value;
        self.top += 1;
        true
    }

    pub fn pop(&mut self) -> Option<i32> {
        if self.is_empty() {
            return None;
        }

        // 出栈时先让 top 回到当前栈顶元素的位置。
        self.top -= 1;
        Some(self.data[self.top])
    }

    pub fn top(&self) -> Option<i32> {
        if self.is_empty() {
            return None;
        }

        Some(self.data[self.top - 1])
    }

    pub fn output(&self) -> Vec<i32> {
        self.data[..self.top].to_vec()
    }
}

impl Default for SeqStack {
    fn default() -> Self {
        Self::new()
    }
}
