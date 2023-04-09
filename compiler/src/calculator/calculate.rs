use crate::parser::evaluate::evaluate;
use crate::parser::rpn::generate_rpn;
use crate::tokenizer::token::Token;
use crate::tokenizer::tokenizer::tokenize;

pub fn calculate(expression: &str) -> i32 {
    let tokens = tokenize(expression);
    let rpn = generate_rpn(&tokens);
    let evaluated = evaluate(&rpn);

    match evaluated {
        Token::Value(value) => value,
        _ => panic!("Unexpected error"),
    }
}

mod tests {
    use super::*;

    #[test]
    fn calculator_calculate() {
        let expression = "(5 + 3) * 12 ^ 2 - 4 / 2";
        let expected = 1150;

        let value = calculate(expression);

        assert_eq!(value, expected);
    }
}
