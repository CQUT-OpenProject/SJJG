#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatientNode {
    pub name: String,
    pub next: Option<Box<PatientNode>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatientQueue {
    head: Option<Box<PatientNode>>,
    len: usize,
}

impl PatientQueue {
    pub fn new() -> Self {
        Self { head: None, len: 0 }
    }

    pub fn enqueue(&mut self, name: &str) {
        let node = Box::new(PatientNode {
            name: name.to_string(),
            next: None,
        });

        match self.head.as_mut() {
            None => {
                self.head = Some(node);
            }
            Some(curr) => {
                let mut tail = curr;
                // 新到达的病人总是排在队尾。
                while let Some(ref mut next) = tail.next {
                    tail = next;
                }
                tail.next = Some(node);
            }
        }

        self.len += 1;
    }

    pub fn dequeue(&mut self) -> Option<String> {
        let mut node = self.head.take()?;
        self.head = node.next.take();
        self.len -= 1;
        Some(node.name)
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn output(&self) -> Vec<String> {
        let mut result = Vec::new();
        let mut curr = self.head.as_ref();

        while let Some(node) = curr {
            result.push(node.name.clone());
            curr = node.next.as_ref();
        }

        result
    }
}

impl Default for PatientQueue {
    fn default() -> Self {
        Self::new()
    }
}

pub fn simulate_hospital(commands: &[&str]) -> Vec<String> {
    let mut queue = PatientQueue::new();
    let mut result = Vec::new();

    for cmd in commands {
        if let Some(name) = cmd.strip_prefix("arrive ") {
            // arrive 命令表示有病人到达，直接进入等待队列。
            queue.enqueue(name);
            result.push(format!(
                "病人 {} 到达，当前等待: {:?}",
                name,
                queue.output()
            ));
        } else if *cmd == "next" {
            // next 命令表示护士叫下一位病人就诊，所以让队头出队。
            match queue.dequeue() {
                Some(name) => result.push(format!("病人 {} 进入诊室", name)),
                None => result.push("当前没有病人等待".to_string()),
            }
        } else if *cmd == "status" {
            result.push(format!(
                "剩余等待人数: {}，队列: {:?}",
                queue.len(),
                queue.output()
            ));
        }
    }

    result
}
