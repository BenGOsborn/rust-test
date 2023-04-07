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
        let temp = Some(Box::new(Node::new(data)));
        temp.next = self.head;

        self.head = temp;
    }
}

fn main() {
    // let mut head = Node::new(3);
    let mut stack: Stack<i32> = Stack::new();
}
