extern crate compiler;

use compiler::calculator::calculate::calculate;
use std::io;

fn main() {
    println!("Welcome to the basic calculator!");
    println!("Operators supported = {{'+', '-', '*', '/', '^', '(', ')'}}");
    println!("Enter 'exit' to quit");

    loop {
        print!("> ");

        let mut expression = String::new();
        io::stdin()
            .read_line(&mut expression)
            .expect("Failed to read line");

        let value = calculate(&expression);

        println!("{:?} = {:?}", expression, value);
    }
}
