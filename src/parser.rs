use crate::expression::*;
use crate::lexer::*;

pub fn generate_mult(v: &mut Vec<Token>) -> Expression {
    let mut expressions = Vec::<Expression>::new();
    loop {
        match v[0] {
            Token::RParen => { v.remove(0); return Expression::Mult(expressions) },
            _ => expressions.push(generate_expression(v))
        }
    }
}


pub fn generate_add(v: &mut Vec<Token>) -> Expression {
    let mut expressions = Vec::<Expression>::new();
    loop {
        match v[0] {
            Token::RParen => { v.remove(0); return Expression::Add(expressions) },
            _ => expressions.push(generate_expression(v))
        }
    }
}

pub fn generate_expression(mut v: &mut Vec<Token>) -> Expression {
   match v.remove(0) {
       Token::Constant(x) => Expression::Number(x),
       Token::LParen => generate_expression(v),
       Token::RParen => panic!("Right paren handled elsewhere"),
       Token::Oper(Operator::Plus) => generate_add(v), 
       Token::Oper(Operator::Mult) => generate_mult(v),
        _ => panic!("I don't know how to do this yet")
   }
   
}
