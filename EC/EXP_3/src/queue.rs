/// 循环顺序队列
/// 通过牺牲一个存储单元来区分队空与队满
pub struct Queue<T> {
    data: Vec<Option<T>>,
    head: usize,
    tail: usize,
    cap: usize,
}

impl<T> Queue<T> {
    pub fn new(capacity: usize) -> Self {
        Queue {
            data: (0..=capacity).map(|_| None).collect(),
            head: 0,
            tail: 0,
            cap: capacity,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head == self.tail
    }

    pub fn is_full(&self) -> bool {
        (self.tail + 1) % (self.cap + 1) == self.head
    }

    pub fn len(&self) -> usize {
        if self.tail >= self.head {
            self.tail - self.head
        } else {
            self.cap + 1 - (self.head - self.tail)
        }
    }

    pub fn enqueue(&mut self, val: T) -> bool {
        if self.is_full() {
            return false;
        }
        self.data[self.tail] = Some(val);
        self.tail = (self.tail + 1) % (self.cap + 1);
        true
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let v = self.data[self.head].take();
        self.head = (self.head + 1) % (self.cap + 1);
        v
    }

    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            self.data[self.head].as_ref()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enqueue_dequeue_fifo() {
        let mut q: Queue<i32> = Queue::new(4);
        assert!(q.is_empty());
        assert!(q.enqueue(1));
        assert!(q.enqueue(2));
        assert!(q.enqueue(3));
        assert_eq!(q.len(), 3);
        assert_eq!(q.dequeue(), Some(1));
        assert_eq!(q.dequeue(), Some(2));
        assert_eq!(q.dequeue(), Some(3));
        assert!(q.is_empty());
        assert_eq!(q.dequeue(), None);
    }

    #[test]
    fn full_and_wrap() {
        let mut q: Queue<i32> = Queue::new(2);
        assert!(q.enqueue(1));
        assert!(q.enqueue(2));
        // 容量 2，但实际可存 2 个元素（cap+1=3，去掉一个空位）
        assert!(!q.enqueue(3));
        assert_eq!(q.dequeue(), Some(1));
        assert!(q.enqueue(3));
        assert_eq!(q.dequeue(), Some(2));
        assert_eq!(q.dequeue(), Some(3));
        assert!(q.is_empty());
    }
}
