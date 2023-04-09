use crate::stack::stack::Stack;
use crate::tokenizer::token::Token;

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
