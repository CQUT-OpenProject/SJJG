use queue_ex_2::LinkQueue;

fn main() {
    let mut queue = LinkQueue::new();

    queue.enqueue(7);
    queue.enqueue(14);
    queue.enqueue(21);

    println!("链队列内容: {:?}", queue.output());
    println!("出队元素: {:?}", queue.dequeue());
    println!("出队后: {:?}", queue.output());
    println!("链队列是否为空: {}", queue.is_empty());
    println!("链队列是否为满: {}", queue.is_full());
}
