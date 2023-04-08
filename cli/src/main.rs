extern crate compiler;

use compiler::parser::parser::parse;
use compiler::stack::stack::Stack;
use compiler::tokenizer::tokenizer::tokenize;

fn main() {
    // let mut stack: Stack<i32> = Stack::new();

    // stack.push(2);
    // stack.push(4);

    // println!("{:?}", stack.peek());
    // println!("{:?}", stack.length);

    // println!("{:?}", stack.pop());
    // println!("{:?}", stack.pop());
    // println!("{:?}", stack.length);

    // println!("{:?}", stack.pop());

    let expression = "(3 + 5) * 12";
    let tokens = tokenize(expression);
    println!("{:?}", tokens);

    let parsed = parse(&tokens);
    println!("{:?}", parsed);
}
