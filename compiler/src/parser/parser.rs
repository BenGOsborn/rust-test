use crate::stack::stack::Stack;
use crate::tokenizer::token::Token;

fn generate_rpn(tokens: &Vec<Token>) -> Vec<&Token> {
    let mut stack: Stack<&Token> = Stack::new();
    let mut rpn: Vec<&Token> = Vec::new();

    // **** Let us not deal with error handling for now

    for token in tokens {
        match token {
            Token::BracketOpen => stack.push(token),
            Token::BracketClosed => {
                while let Some(item) = stack.pop() {
                    match item {
                        Token::BracketOpen => break,
                        _ => rpn.push(item),
                    }
                }
            }
            Token::OpPower
            | Token::OpMultiply
            | Token::OpDivide
            | Token::OpAdd
            | Token::OpSubtract => {
                // **** We first check if the stack is empty, in which we will push an element to the stack
                // **** If there is already an element on the stack, we check if our new element is greater where we push another one on
                // **** Otherwise, we pop the previous ones off until we find an element that is less where we push it on again
            }
            Token::Value(_) => rpn.push(token),
        }
    }

    // **** Add the remaining elements from the stack to the sequence

    rpn
}

pub fn parse(tokens: &Vec<Token>) -> i32 {
    let rpn = generate_rpn(tokens);

    println!("{:?}", rpn);

    -1
}
