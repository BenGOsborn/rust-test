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

mod tests {
    use super::*;

    #[test]
    fn rpn_generate_rpn() {
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
        let expected = vec![
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

        let rpn = generate_rpn(&tokens);

        assert_eq!(rpn, expected);
    }
}
