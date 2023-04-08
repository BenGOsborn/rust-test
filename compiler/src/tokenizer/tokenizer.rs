use crate::tokenizer::token::Token;

enum Created {
    Created(Token),
    Exists(Token),
}

pub fn tokenize(expression: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    for char in expression.chars() {
        match char {
            '(' => tokens.push(Token::BracketOpen),
            ')' => tokens.push(Token::BracketClosed),
            '^' => tokens.push(Token::OpPower),
            '+' => tokens.push(Token::OpAdd),
            '-' => tokens.push(Token::OpSubtract),
            '*' => tokens.push(Token::OpMultiply),
            '/' => tokens.push(Token::OpDivide),
            '0'..='9' => {
                tokens
                    .last()
                    .map(|token| match token {
                        Token::Value(prev) => {
                            Created::Exists(Token::Value(prev.to_string() + &char.to_string()))
                        }
                        _ => Created::Created(Token::Value(char.to_string())),
                    })
                    .map(|created| match created {
                        Created::Created(token) => tokens.push(token),
                        Created::Exists(token) => {
                            tokens.remove(tokens.len() - 1);
                            tokens.push(token);
                        }
                    });
            }
            _ => {}
        }
    }

    tokens
}
