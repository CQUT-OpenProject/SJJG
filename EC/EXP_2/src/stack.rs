/// 顺序栈，存储当前正在尝试着色的区域下标
pub struct Stack {
    data: Vec<usize>,
    top: usize,
}

impl Stack {
    pub fn new(capacity: usize) -> Self {
        Stack {
            data: vec![0; capacity],
            top: 0,
        }
    }

    pub fn push(&mut self, val: usize) -> bool {
        if self.top >= self.data.len() {
            return false;
        }
        self.data[self.top] = val;
        self.top += 1;
        true
    }

    pub fn pop(&mut self) -> Option<usize> {
        if self.top == 0 {
            return None;
        }
        self.top -= 1;
        Some(self.data[self.top])
    }

    pub fn peek(&self) -> Option<usize> {
        if self.top == 0 {
            None
        } else {
            Some(self.data[self.top - 1])
        }
    }

    pub fn is_empty(&self) -> bool {
        self.top == 0
    }

    pub fn len(&self) -> usize {
        self.top
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_pop_basic() {
        let mut s = Stack::new(4);
        assert!(s.is_empty());
        assert!(s.push(1));
        assert!(s.push(2));
        assert_eq!(s.len(), 2);
        assert_eq!(s.peek(), Some(2));
        assert_eq!(s.pop(), Some(2));
        assert_eq!(s.pop(), Some(1));
        assert!(s.is_empty());
        assert_eq!(s.pop(), None);
    }

    #[test]
    fn overflow() {
        let mut s = Stack::new(2);
        assert!(s.push(1));
        assert!(s.push(2));
        assert!(!s.push(3));
    }
}
