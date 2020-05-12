use std::collections::HashMap;
use interpreter::environment::*;
use interpreter::evaluator::*;
use interpreter::expression::*;
use interpreter::parser::*;
use interpreter::lexer::*;

fn main() {
    
    let environment = Environment::new();
    let expression_string = "(+ 1 2 3 (* 4 5 6))";
    let value = evaluate(&environment, &generate_expression(&mut tokenize(expression_string)));
    println!("We computed {}", value);
}
