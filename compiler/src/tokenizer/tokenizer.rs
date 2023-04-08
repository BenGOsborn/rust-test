use crate::tokenizer::token::Token;

pub fn tokenize(sequence: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    for char in sequence.chars() {
        match char {
            '(' => tokens.push(Token::BracketOpen),
            ')' => tokens.push(Token::BracketClosed),
            '^' => tokens.push(Token::OpPower),
            '+' => tokens.push(Token::OpAdd),
            '-' => tokens.push(Token::OpSubtract),
            '*' => tokens.push(Token::OpMultiply),
            '/' => tokens.push(Token::OpDivide),
            '0'..='9' => tokens.push(Token::Value(char as i32 - '0' as i32)),
            _ => {}
        }
    }

    return tokens;
}
