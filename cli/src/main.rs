extern crate compiler;

use compiler::parser::parser::{evaluate, generate_rpn};
use compiler::tokenizer::tokenizer::tokenize;

fn main() {
    let expression = "(3 + 5) * 12";

    let tokens = tokenize(expression);
    println!("Tokens: {:?}", tokens);

    let rpn = generate_rpn(&tokens);
    println!("RPN: {:?}", rpn);

    let evaluated = evaluate(&rpn);
    println!("Evaluated: {:?}", evaluated);
}
