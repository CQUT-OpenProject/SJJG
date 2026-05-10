pub const MAX: usize = 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CircularQueue {
    front: usize,
    rear: usize,
    data: [i32; MAX],
}

impl CircularQueue {
    pub fn new() -> Self {
        Self {
            front: 0,
            rear: 0,
            data: [0; MAX],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.front == self.rear
    }

    pub fn is_full(&self) -> bool {
        // 循环队列预留一个空位置，用来区分“空”和“满”。
        (self.rear + 1) % MAX == self.front
    }

    pub fn enqueue(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }

        // rear 总是指向下一个可以入队的位置。
        self.data[self.rear] = value;
        self.rear = (self.rear + 1) % MAX;
        true
    }

    pub fn dequeue(&mut self) -> Option<i32> {
        if self.is_empty() {
            return None;
        }

        // front 指向当前队头元素，取出后再向后移动一格。
        let value = self.data[self.front];
        self.front = (self.front + 1) % MAX;
        Some(value)
    }

    pub fn output(&self) -> Vec<i32> {
        let mut result = Vec::new();
        let mut idx = self.front;

        // 从 front 开始一直走到 rear，把当前队列内容依次取出。
        while idx != self.rear {
            result.push(self.data[idx]);
            idx = (idx + 1) % MAX;
        }

        result
    }
}

impl Default for CircularQueue {
    fn default() -> Self {
        Self::new()
    }
}
