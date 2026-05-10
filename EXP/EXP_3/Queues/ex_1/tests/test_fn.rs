use queue_ex_1::{CircularQueue, MAX};

#[test]
fn circular_queue_supports_basic_operations() {
    let mut queue = CircularQueue::new();

    assert!(queue.is_empty());
    assert!(queue.enqueue(1));
    assert!(queue.enqueue(2));
    assert!(queue.enqueue(3));

    assert_eq!(queue.output(), vec![1, 2, 3]);
    assert_eq!(queue.dequeue(), Some(1));
    assert_eq!(queue.dequeue(), Some(2));
    assert_eq!(queue.output(), vec![3]);
    assert!(!queue.is_full());
}

#[test]
fn circular_queue_uses_one_slot_to_detect_full() {
    let mut queue = CircularQueue::new();

    for idx in 0..(MAX - 1) {
        assert!(queue.enqueue(idx as i32));
    }

    assert!(queue.is_full());
    assert!(!queue.enqueue(9999));
    assert_eq!(queue.dequeue(), Some(0));
    assert!(queue.enqueue(9999));
}
