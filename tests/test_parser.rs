
#[cfg(test)]

extern crate interpreter;
pub use interpreter::expression::*;
pub use interpreter::evaluator::*;
pub use interpreter::environment::*;
pub use interpreter::lexer::*;
pub use interpreter::parser::*;

mod parser_test {
    use super::*;

    #[test]
    fn [parser_exists(){
        assert_eq!(1, 1);
    }

    #[test]
    fn parser_exists() {
        assert_eq!(1, 1);
    }

    #[test]
    fn converts_one_constant_token_into_number_expression() {
        //arrange
        let mut tokens = vec![Token::Constant(5.0)];
        let environment = Environment::new();
        //act
        let expression = generate_expression(&mut tokens);
        //assert
        assert_eq!(evaluate(&environment, expression), 5.0);
    }
    #[test]
    fn parser_adds_two_numbers_together() {
        //arrange
        let mut tokens = vec![Token::LParen, Token::Oper(Operator::Plus), Token::Constant(5.0), Token::Constant(3.0), Token::RParen];
        let environment = Environment::new();
        //act
        let expression = generate_expression(&mut tokens);
        //assert
        assert_eq!(evaluate(&environment, &expression), 8.0);
    }
    
    #[test]
    fn parser_multiply_two_numbers_together() {
        //arrange
        let mut tokens = vec![Token::LParen, Token::Oper(Operator::Plus), Token::Constant(5.0), Token::Constant(3.0), Token::RParen];
        let environment = Environment::new();
        //act
        let expression = generate_expression(&mut tokens);
        //assert
        assert_eq!(evaluate(&environment, &expression), 15.0);
    }

    #[test]
    fn parser_works_recursively() { 
        //arrange (* 5 3 (+ 1 1))
        let mut tokens = vec![Token::LParen, Token::Oper(Operator::Mult), Token::Constant(5.0), Token::Constant(3.0), Token::LParen, Token::Oper(Operator::Add), Token::Constant(1.0), Token::Constant(1.0), Token::RParen, Token::RParen, ];
        let environment = Environment::new();
        //act
        let expression = generate_expression(&mut tokens);
        //assert
        assert_eq!(evaluate(&environment, &expression), 30.0);
    }
}

