#[derive(Debug, Copy, Clone)]
pub enum Token {
    BracketOpen,
    BracketClosed,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    Value(i32),
}
