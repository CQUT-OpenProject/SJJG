use stack_ex_1::{SeqStack, MAX};

#[test]
fn sequential_stack_supports_basic_operations() {
    let mut stack = SeqStack::new();

    assert!(stack.is_empty());
    assert_eq!(stack.top(), None);

    assert!(stack.push(10));
    assert!(stack.push(20));
    assert!(stack.push(30));

    assert_eq!(stack.output(), vec![10, 20, 30]);
    assert_eq!(stack.top(), Some(30));
    assert_eq!(stack.pop(), Some(30));
    assert_eq!(stack.pop(), Some(20));
    assert_eq!(stack.top(), Some(10));
    assert!(!stack.is_full());
}

#[test]
fn sequential_stack_detects_full_and_empty() {
    let mut stack = SeqStack::new();

    for idx in 0..MAX {
        assert!(stack.push(idx as i32));
    }

    assert!(stack.is_full());
    assert!(!stack.push(2048));

    for _ in 0..MAX {
        assert!(stack.pop().is_some());
    }

    assert!(stack.is_empty());
    assert_eq!(stack.pop(), None);
}
