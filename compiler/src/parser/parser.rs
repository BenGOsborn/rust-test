use crate::stack::stack::Stack;
use crate::tokenizer::token::Token;

enum StackOption {
    Pop,
    Push,
}

pub fn generate_rpn(tokens: &Vec<Token>) -> Vec<&Token> {
    let mut stack: Stack<&Token> = Stack::new();
    let mut rpn: Vec<&Token> = Vec::new();

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
                // If the stack is empty or the new operator takes precedence, we push the operator to the stack
                // Otherwise, pop all operators that take precedence on the stack off before adding next
                loop {
                    let value = stack.peek().map_or(StackOption::Push, |stack_token| {
                        if token.compare() > stack_token.compare() {
                            StackOption::Push
                        } else {
                            StackOption::Pop
                        }
                    });

                    match value {
                        StackOption::Push => {
                            stack.push(token);
                            break;
                        }
                        StackOption::Pop => {
                            stack.pop().map(|token| rpn.push(token));
                        }
                    }
                }
            }
            Token::Value(_) => rpn.push(token),
        }
    }

    // Add the remaining elements from the stack to the sequence
    while let Some(item) = stack.pop() {
        rpn.push(item);
    }

    rpn
}

pub fn evaluate(rpn: &Vec<&Token>) -> Token {
    let mut stack: Stack<Token> = Stack::new();

    for token in rpn {
        match token {
            Token::OpAdd
            | Token::OpSubtract
            | Token::OpMultiply
            | Token::OpDivide
            | Token::OpPower => {
                let operand1 = stack.pop().expect("Expected operand 1");
                let operand2 = stack.pop().expect("Expected operand 2");

                match (operand1, operand2) {
                    (Token::Value(v1), Token::Value(v2)) => {
                        let val = match token {
                            Token::OpAdd => v1 + v2,
                            Token::OpSubtract => v1 - v2,
                            Token::OpMultiply => v1 * v2,
                            Token::OpDivide => v1 / v2,
                            Token::OpPower => v1 ^ v2,
                            _ => panic!("Fatal error"),
                        };

                        stack.push(Token::Value(val));
                    }
                    _ => panic!(
                        "Invalid tokens for operation '{:?}': '{:?}' and '{:?}'",
                        token, operand1, operand2
                    ),
                }
            }
            Token::Value(_) => stack.push(**token),
            _ => panic!("Invalid token '{:?}'", token),
        }
    }

    stack.pop().unwrap()
}
