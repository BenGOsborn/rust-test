#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    BracketOpen,
    BracketClosed,
    OpPower,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    Value(i32),
}

impl Token {
    pub fn compare(&self) -> i32 {
        match self {
            Token::BracketClosed => 4,
            Token::OpPower => 3,
            Token::OpMultiply => 2,
            Token::OpDivide => 2,
            Token::OpAdd => 1,
            Token::OpSubtract => 1,
            Token::Value(_) => 0,
            Token::BracketOpen => -1, // Needs to be last so other operators / values never pop it off the stack
        }
    }
}
