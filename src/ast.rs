use crate::token::Token;
use std::fmt;
pub trait Node {
    fn token_literal(&self) -> &str;
}

pub trait Statement: Node {
    fn statement_node(&self);
}

pub trait Expression: Node {
    fn expression_node(&self);
}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> &str {
        match self.statements.len() {
            0 => "",
            _ => self.statements[0].token_literal(),
        }
    }
}

pub struct ReturnStatement {
    pub token: Token,
    //pub return_value: Expression,
}

impl Statement for ReturnStatement {
    fn statement_node(&self) {
        unimplemented!();
    }
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> &str {
        self.token.literal.as_str()
    }
}

pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    //value: Expression,
}

impl Statement for LetStatement {
    fn statement_node(&self) {
        unimplemented!();
    }
}

impl Node for LetStatement {
    fn token_literal(&self) -> &str {
        self.token.literal.as_str()
    }
}

pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Expression for Identifier {
    fn expression_node(&self) {
        unimplemented!();
    }
}

impl Node for Identifier {
    fn token_literal(&self) -> &str {
        self.token.literal.as_str()
    }
}
