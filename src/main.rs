struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node {
            data: data,
            next: None,
        }
    }
}

struct Stack<T> {
    length: usize,
    head: Option<Box<Node<T>>>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack {
            length: 0,
            head: None,
        }
    }

    fn push(&mut self, data: T) {
        let mut temp = Box::new(Node::new(data));
        temp.next = self.head.take();

        self.head = Some(temp);
        self.length += 1;
    }

    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|boxed_node| {
            let node = boxed_node;

            self.head = node.next;
            self.length -= 1;

            node.data
        })
    }
}

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
