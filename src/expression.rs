

#[derive(Clone)]
#[derive(Debug)]
pub enum Expression {
    Add(Vec<Expression>),
    Mult(Vec<Expression>),
    Subtract(Vec<Expression>),
    Variable(String),
    Number(f64),
}
