#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListNode {
    pub data: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(data: i32) -> Self {
        Self { data, next: None }
    }
}

pub fn from_slice(data: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;

    // 倒着头插，最后得到的链表顺序才和切片顺序一致。
    for &value in data.iter().rev() {
        head = Some(Box::new(ListNode {
            data: value,
            next: head,
        }));
    }

    head
}

pub fn to_vec(head: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut curr = head.as_ref();

    while let Some(node) = curr {
        result.push(node.data);
        curr = node.next.as_ref();
    }

    result
}

pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut stack = Vec::new();
    let mut curr = head.take();

    // 先把链表数据顺序压栈，利用后进先出得到逆序结果。
    while let Some(mut node) = curr {
        stack.push(node.data);
        curr = node.next.take();
    }

    let mut new_head = None;
    for value in stack {
        // 把出栈元素不断头插到新链表，形成逆置后的结果。
        new_head = Some(Box::new(ListNode {
            data: value,
            next: new_head,
        }));
    }

    new_head
}
