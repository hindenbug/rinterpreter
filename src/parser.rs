use crate::ast::{Identifier, LetStatement, Program, ReturnStatement, Statement};
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
    errors: Vec<ParseError>,
}

impl<'a> Parser<'a> {
    fn new(mut lexer: Lexer<'a>) -> Self {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Parser {
            lexer,
            current_token: current_token,
            peek_token: peek_token,
            errors: vec![],
        }
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut statements: Vec<Box<dyn Statement>> = vec![];
        while !self.current_token_is(&TokenType::EOF) {
            match self.parse_statement() {
                Ok(stmt) => statements.push(stmt),
                Err(error) => self.errors.push(error),
            }
            self.next_token();
        }

        // handle errors
        Program { statements }
    }

    fn parse_statement(&mut self) -> Result<Box<dyn Statement>, ParseError> {
        match self.current_token.token_type {
            TokenType::LET => self.parse_let_statement(),
            TokenType::RETURN => self.parse_return_statement(),
            _ => Err(ParseError {
                message: "Some error".to_string(),
            }),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Box<dyn Statement>, ParseError> {
        let token = self.current_token.clone();

        if !self.expect_peek(&TokenType::IDENT) {
            return Err(ParseError {
                message: format!(
                    "expected next token to be IDENT, got {:?} instead.",
                    self.peek_token.token_type
                ),
            });
        }

        let identifier = Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        };

        if !self.expect_peek(&TokenType::ASSIGN) {
            return Err(ParseError {
                message: format!(
                    "expected next token to be ASSIGN, got {:?} instead.",
                    self.peek_token.token_type
                ),
            });
        }

        // TODO: We're skipping the expressions until we encounter a semicolon
        while !self.current_token_is(&TokenType::SEMICOLON) {
            self.next_token();
        }

        Ok(Box::new(LetStatement {
            token,
            name: identifier,
            value: None,
        }))
    }

    fn parse_return_statement(&mut self) -> Result<Box<dyn Statement>, ParseError> {
        let token = self.current_token.clone();

        while !self.current_token_is(&TokenType::SEMICOLON) {
            self.next_token();
        }

        Ok(Box::new(ReturnStatement { token }))
    }

    fn current_token_is(&self, t: &TokenType) -> bool {
        self.current_token.token_type == *t
    }

    fn peek_token_is(&self, t: &TokenType) -> bool {
        self.peek_token.token_type == *t
    }

    fn expect_peek(&mut self, t: &TokenType) -> bool {
        return if self.peek_token_is(t) {
            self.next_token();
            true
        } else {
            self.errors.push(ParseError {
                message: format!(
                    "expected next token to be {:?}, got {:?} instead",
                    t, self.peek_token.token_type
                ),
            });
            false
        };
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

        let program = parser.parse_program();

        assert_eq!(3, program.statements.len());
        for s in program.statements {
            assert_eq!("let".to_owned(), s.token_literal());
        }
    }

    #[test]
    fn test_invalid_statement() {
        let input = r#"
let x = 5;
let = 10;
let foobar = 838383;
        "#;

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        parser.parse_program();
        check_parser_errors(&parser);

        assert_eq!(parser.errors.len(), 5);
    }

    #[test]
    fn test_return_statements() {
        let input = r#"
return 5;
return 10;
return 993322;
            "#;
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(&parser);

        assert_eq!(3, program.statements.len());
        for s in program.statements {
            assert_eq!("return".to_owned(), s.token_literal());
        }
    }

    fn check_parser_errors(parser: &Parser) {
        if parser.errors.is_empty() {
            return;
        }

        println!("Parser has {} errors", parser.errors.len());
        parser
            .errors
            .iter()
            .for_each(|err| eprintln!("parser error: {}", err));
    }
}
