use crate::stack::node::Node;

pub struct Stack<T> {
    pub length: u32,
    pub head: Option<Box<Node<T>>>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            length: 0,
            head: None,
        }
    }

    pub fn push(&mut self, data: T) {
        let mut temp = Box::new(Node::new(data));
        temp.next = self.head.take();

        self.head = Some(temp);
        self.length += 1;
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|boxed_node| {
            let node = boxed_node;

            self.head = node.next;
            self.length -= 1;

            node.data
        })
    }
}
