use queue_ex_1::CircularQueue;

fn main() {
    let mut queue = CircularQueue::new();

    println!("循环队列是否为空: {}", queue.is_empty());
    queue.enqueue(11);
    queue.enqueue(22);
    queue.enqueue(33);

    println!("入队后: {:?}", queue.output());
    println!("出队元素: {:?}", queue.dequeue());
    println!("再次出队后: {:?}", queue.output());
    println!("循环队列是否为满: {}", queue.is_full());
}
