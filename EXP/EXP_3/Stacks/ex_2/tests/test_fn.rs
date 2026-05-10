use stack_ex_2::LinkStack;

#[test]
fn linked_stack_supports_basic_operations() {
    let mut stack = LinkStack::new();

    assert!(stack.is_empty());

    stack.push(1);
    stack.push(2);
    stack.push(3);

    assert_eq!(stack.output(), vec![3, 2, 1]);
    assert_eq!(stack.top(), Some(3));
    assert_eq!(stack.len(), 3);
    assert_eq!(stack.pop(), Some(3));
    assert_eq!(stack.pop(), Some(2));
    assert_eq!(stack.pop(), Some(1));
    assert_eq!(stack.pop(), None);
    assert!(stack.is_empty());
}

#[test]
fn linked_stack_is_not_reported_full() {
    let mut stack = LinkStack::new();

    for idx in 0..200 {
        stack.push(idx);
    }

    assert!(!stack.is_full());
    assert_eq!(stack.top(), Some(199));
}
