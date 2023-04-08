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
