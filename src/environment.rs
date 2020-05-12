use std::collections::HashMap;
use crate::expression::*;

pub struct Environment {
        hash: HashMap<String, Expression>
}

impl Environment {
        pub fn define(self: &mut Environment, key: String, value: Expression) {
                    self.hash.insert(key, value);
                        }

            pub fn resolve(self: &Environment, expression: &Expression) -> &Expression {
                        println!("resolving {:?}", expression);
                                if let Expression::Variable(name) = expression {
                                                if let Some(value) = self.hash.get(name) {
                                                                    value
                                                                                    } else {
                                                                                                        panic!("variable not found");
                                                                                                                    }
                                                        } else {
                                                                        panic!("Trying to resolve an expression({:?}) that is not a variable", expression);
                                                                                }
                                    }
                pub fn new() -> Environment {
                            Environment{hash: HashMap::new()}
                                }
}
