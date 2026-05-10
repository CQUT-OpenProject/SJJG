use queue_ex_2::LinkQueue;

#[test]
fn linked_queue_supports_fifo() {
    let mut queue = LinkQueue::new();

    queue.enqueue(10);
    queue.enqueue(20);
    queue.enqueue(30);

    assert_eq!(queue.output(), vec![10, 20, 30]);
    assert_eq!(queue.dequeue(), Some(10));
    assert_eq!(queue.dequeue(), Some(20));
    assert_eq!(queue.output(), vec![30]);
    assert_eq!(queue.len(), 1);
}

#[test]
fn linked_queue_is_never_reported_full() {
    let mut queue = LinkQueue::new();

    for idx in 0..100 {
        queue.enqueue(idx);
    }

    assert!(!queue.is_full());
    assert_eq!(queue.dequeue(), Some(0));
}
