#[cfg(test)]

extern crate interpreter;
pub use interpreter::expression::*;
pub use interpreter::evaluator::*;
pub use interpreter::environment::*;
pub use interpreter::lexer::*;

mod lexer_test {
    use super::*;

    #[test]
    fn lexer_exists(){
        assert_eq!(1, 1);
    }

    #[test]
    fn lexer_creates_empty_tokens_from_empty_string() {
        //arrange
        let src_string = "";
        //act
        let tokens = Tokenize(src_string);
        //assert
        assert_eq!(tokens.len(), 0);
    }
}

#[test]
fn lexer_creates_parenthesis_from_empty_parens(){
    //arrange
    let src_string = "()";
    //act
    let tokens = Tokenize(&src_string);
    //assert
    assert_eq!(tokens.len(), 2);

}

#[test]
fn lexer_creates_plus_or_mult_from_characters() {
    //arrang
    let src_string = "+*";
    //act
    let tokens = Tokenize(&src_string);
    //assert
    assert_eq!(tokens.len(), 2);
}

#[test]
fn lexer_creates_tokens_for_reasonable_statement() {
    //arrange
    let src_string = "(+ 3 5)";
    //act
    let tokens = Tokenize(&src_string);
    //assert
    assert_eq!(tokens.len(), 5);
}
