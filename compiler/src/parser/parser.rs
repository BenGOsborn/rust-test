use std::ops::Add;

use crate::stack::stack::Stack;
use crate::tokenizer::token::Token;

pub fn parse(tokens: &Vec<Token>) -> i32 {
    let mut stack: Stack<&Token> = Stack::new();
    let mut sequence: Vec<&str> = Vec::new();

    // **** Now I also need a way of keeping track of numbers from their character form

    for token in tokens {
        match token {
            Token::BracketOpen => stack.push(token),
            Token::BracketClosed => {}
            Token::OpPower => {}
            Token::OpMultiply => {}
            Token::OpDivide => {}
            Token::OpAdd => {}
            Token::OpSubtract => {}
            Token::Value(c) => {}
        }
    }

    println!("{:?}", sequence);

    // **** This will require looping through the tokens, creating a stack structure, validating the stack as we go, and returning the output
    // **** In addition, enable the option to debug where the value will be printed

    -1
}
