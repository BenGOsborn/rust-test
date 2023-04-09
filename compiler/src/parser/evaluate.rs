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
                            Token::OpAdd => v2 + v1,
                            Token::OpSubtract => v2 - v1,
                            Token::OpMultiply => v2 * v1,
                            Token::OpDivide => v2 / v1,
                            Token::OpPower => v2 ^ v1,
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
