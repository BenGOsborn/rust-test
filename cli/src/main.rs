extern crate stack;

use stack::stack::stack::Stack;

fn main() {
    // let mut head = Node::new(3);
    let mut stack: Stack<i32> = Stack::new();

    stack.push(2);
    stack.push(4);

    println!("{:?}", stack.peek());
    println!("{:?}", stack.length);

    println!("{:?}", stack.pop());
    println!("{:?}", stack.pop());
    println!("{:?}", stack.length);

    println!("{:?}", stack.pop());
}
