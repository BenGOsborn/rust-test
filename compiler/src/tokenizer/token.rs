#[derive(Debug)]
pub enum Token {
    BracketOpen,
    BracketClosed,
    OpPower,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    Value(String),
}

impl Token {
    pub fn compare(&self) -> i32 {
        match self {
            Token::BracketOpen => 4,
            Token::BracketClosed => 4,
            Token::OpPower => 3,
            Token::OpMultiply => 2,
            Token::OpDivide => 2,
            Token::OpAdd => 1,
            Token::OpSubtract => 1,
            Token::Value(_) => 0,
        }
    }
}
