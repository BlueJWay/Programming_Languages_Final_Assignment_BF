use crate::expression::*;
use crate::environment::*;

pub fn evaluate_addition(environment: &Environment, add: &Expression) -> f64 {
    if let Expression::Add(expressions) = add {
        let iter = expressions.iter();
        iter.fold(0.0, |total, next| total + evaluate(environment, next))
    } else {
        panic!("Addition not provided")
    }
}

pub fn evaluate_multiplication(environment: &Environment, mult: &Expression) -> f64 {
    if let Expression::Mult(expressions) = mult {
        let iter = expressions.iter();
        iter.fold(1.0, |total, next| total * evaluate(environment, next))
    } else {
        panic!("Mult not provided")
    }
}

pub fn evaluate_subtraction(environment: &Environment, sub: &Expression) -> f64 {
    if let Expression::Subtract(expressions) = sub {
        let mut iter = expressions.iter();
        let first = iter.next().unwrap();
        iter.fold(evaluate(environment, first),
                |total, next| total - evaluate(environment, next))
    } else {
        panic!("Subtract not provided")
    }
}

pub fn evaluate_variable<'a>(environment: &'a Environment, var: &Expression) -> &'a Expression {
    if let Expression::Variable(_) = var {
        environment.resolve(var)
    } else {
        panic!("Variable not provided ({:?})", var)
    }
}

pub fn evaluate(environment: &Environment, expression: &Expression) -> f64 {
    match expression {
        Expression::Add(_) => evaluate_addition(environment, expression),
        Expression::Mult(_) => evaluate_multiplication(environment, expression),
        Expression::Subtract(_) => evaluate_subtraction(environment, expression),
        Expression::Variable(_) => evaluate(environment,
            &evaluate_variable(environment, &expression)),
        Expression::Number(val) => *val,
    }
}


