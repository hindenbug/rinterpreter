use crate::ast::{Identifier, LetStatement, Program, Statement};
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};
use std::fmt;

#[derive(Debug)]
pub struct ParseError {
    message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[derive(Debug)]
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    fn new(mut lexer: Lexer<'a>) -> Self {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Parser {
            lexer,
            current_token: current_token,
            peek_token: peek_token,
        }
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut statements: Vec<Box<dyn Statement>> = vec![];
        while !self.current_token_is(&TokenType::EOF) {
            match self.parse_statement() {
                Ok(stmt) => statements.push(stmt),
                Err(error) => break,
            }
            self.next_token();
        }
        Ok(Program { statements })
    }

    fn parse_statement(&mut self) -> Result<Box<dyn Statement>, ParseError> {
        match self.current_token.token_type {
            TokenType::LET => self.parse_let_statement(),
            _ => Err(ParseError {
                message: "Unexpected Token".to_string(),
            }),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Box<dyn Statement>, ParseError> {
        let token = self.current_token.clone();

        if !self.expect_peek(&TokenType::IDENT) {
            return Err(ParseError {
                message: "Let statement parse error".to_string(),
            });
        }

        println!("{:?}", self.current_token);

        let identifier = Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        };

        if !self.expect_peek(&TokenType::ASSIGN) {
            return Err(ParseError {
                message: "Let statement parse error".to_string(),
            });
        }

        // TODO: We're skipping the expressions until we // encounter a semicolon
        while !self.current_token_is(&TokenType::SEMICOLON) {
            self.next_token();
        }

        Ok(Box::new(LetStatement {
            token: token,
            name: identifier,
        }))
    }

    fn current_token_is(&self, t: &TokenType) -> bool {
        self.current_token.token_type == *t
    }

    fn peek_token_is(&self, t: &TokenType) -> bool {
        self.peek_token.token_type == *t
    }

    fn expect_peek(&mut self, t: &TokenType) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            return true;
        } else {
            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_let_statements() {
        let input = r#"
let x = 5;
let y = 10;
let foobar = 838383;
            "#;
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        println!("{:?}", parser);
        match parser.parse_program() {
            Ok(p) => {
                assert_eq!(3, p.statements.len());
                for s in p.statements {
                    assert_eq!("let".to_owned(), s.token_literal());
                }
            }
            Err(err) => panic!("{}", err),
        }
    }
}
