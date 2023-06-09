extern crate compiler;

use compiler::calculator::calculate::calculate;
use std::io::{self, Write};

fn main() {
    println!("Welcome to the basic calculator!");
    println!("Operators supported = {{'+', '-', '*', '/', '^', '(', ')'}}");
    println!("Enter 'exit' to quit");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut expression = String::new();
        io::stdin()
            .read_line(&mut expression)
            .expect("Failed to read line");

        let expression = expression.trim();

        if expression == String::from("exit") {
            println!("Goodbye!");
            break;
        }

        let value = calculate(&expression);

        println!("{:?} = {:?}", expression, value);
    }
}
