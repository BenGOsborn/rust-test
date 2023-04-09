extern crate compiler;

use compiler::calculator::calculate::calculate;

fn main() {
    let expression = "(3 + 5) * 12 - 196";

    let value = calculate(expression);

    println!("{:?} = {:?}", expression, value);
}
