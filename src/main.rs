struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>> ,
}

impl <T> Node<T> {
    fn new(data: T, next: Option<Box<Node<T>>>) {

    }
}

fn main() {
    println!("Hello, world!");
}
