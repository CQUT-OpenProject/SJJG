use stack_ex_2::LinkStack;

fn main() {
    let mut stack = LinkStack::new();

    stack.push(5);
    stack.push(15);
    stack.push(25);

    println!("链栈内容(栈顶到栈底): {:?}", stack.output());
    println!("当前栈顶: {:?}", stack.top());
    println!("弹出元素: {:?}", stack.pop());
    println!("弹出后链栈: {:?}", stack.output());
    println!("链栈是否为空: {}", stack.is_empty());
    println!("链栈是否为满: {}", stack.is_full());
}
