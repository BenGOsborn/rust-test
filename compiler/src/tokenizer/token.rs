#[derive(Debug, Copy, Clone)]
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
