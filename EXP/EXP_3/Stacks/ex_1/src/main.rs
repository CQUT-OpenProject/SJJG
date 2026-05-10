use stack_ex_1::SeqStack;

fn main() {
    let mut stack = SeqStack::new();

    println!("顺序栈是否为空: {}", stack.is_empty());
    stack.push(10);
    stack.push(20);
    stack.push(30);

    println!("入栈后: {:?}", stack.output());
    println!("栈顶元素: {:?}", stack.top());
    println!("弹出元素: {:?}", stack.pop());
    println!("弹出后: {:?}", stack.output());
    println!("顺序栈是否已满: {}", stack.is_full());
}
