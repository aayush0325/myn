use super::{Expression, Node, Statement};
use crate::token::Token;

pub struct Identifier {
    pub token: Token,
    pub value: String,
}

pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,

    // Here we're using a Box<dyn Expression> because we
    // don't know it's size at compile time.
    // using "dyn" creates a heap-allocated object with it's
    // vtabels that allows for polymorphism
    pub value: Box<dyn Expression>,
}

// Implement Node for Identifier
impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Expression for Identifier {}

// Implement Node for LetStatement
impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Statement for LetStatement {}
