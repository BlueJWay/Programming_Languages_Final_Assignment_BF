#[derive(Debug)]
#[derive(Clone)]

pub enum Operator {
    Plus,
    Mult
}

#[derive(Debug)]
#[derive(Clone)]

pub enum Token {
    LParen,
    RParen,
    Oper(Operator),
    Constant(f64)
}

pub fn Tokenize(s: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::<Token>::new();
    for c in s.chars() {
        match c {
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            '+' => tokens.push(Token::Oper(Operator::Plus)),
            '*' => tokens.push(Token::Oper(Operator::Mult)),
            '1' => tokens.push(Token::Constant(1.0)),
            '2' => tokens.push(Token::Constant(2.0)),
            '3' => tokens.push(Token::Constant(3.0)),
            '4' => tokens.push(Token::Constant(4.0)),
            '5' => tokens.push(Token::Constant(5.0)),
            '6' => tokens.push(Token::Constant(6.0)),
            '7' => tokens.push(Token::Constant(7.0)),
            '8' => tokens.push(Token::Constant(8.0)),
            '9' => tokens.push(Token::Constant(9.0)),
            '0' => tokens.push(Token::Constant(0.0)),
            _ => { }
    }
    return tokens;    
}
}
