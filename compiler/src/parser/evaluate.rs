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
                            Token::OpPower => v2.pow(v1.try_into().unwrap()),
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

mod tests {
    use super::*;

    #[test]
    fn parser_evaluate() {
        let tokens = vec![
            Token::BracketOpen,
            Token::Value(5),
            Token::OpAdd,
            Token::Value(3),
            Token::BracketClosed,
            Token::OpMultiply,
            Token::Value(12),
            Token::OpPower,
            Token::Value(2),
            Token::OpSubtract,
            Token::Value(4),
            Token::OpDivide,
            Token::Value(2),
        ];
        let rpn = vec![
            &tokens[1],
            &tokens[3],
            &tokens[2],
            &tokens[6],
            &tokens[8],
            &tokens[7],
            &tokens[5],
            &tokens[10],
            &tokens[12],
            &tokens[11],
            &tokens[9],
        ];
        let expected = Token::Value(1150);

        let value = evaluate(&rpn);

        assert_eq!(value, expected);
    }
}
