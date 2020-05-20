use crate::ast::{
    Expression, Identifier, InfixExpression, IntegerLiteral, LetStatement, PrefixExpression,
    Program, ReturnStatement, Statement,
};
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};
use std::fmt;
use std::thread::current;

#[derive(Debug)]
pub struct ParseError {
    message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub enum Precedence {
    LOWEST,
    EQUALS,      // ==
    LESSGREATER, // < OR >
    SUM,         // +
    PRODUCT,     // *
    PREFIX,      // -X OR !X
    CALL,        // func()
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
            current_token,
            peek_token,
            errors: vec![],
        }
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut statements: Vec<Box<Statement>> = vec![];
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

    fn parse_statement(&mut self) -> Result<Box<Statement>, ParseError> {
        Ok(match self.current_token.token_type {
            TokenType::LET => Box::new(self.parse_let_statement()?),
            TokenType::RETURN => Box::new(self.parse_return_statement()?),
            _ => Box::new(self.parse_expression_statement()?),
        })
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, ParseError> {
        let expression = match self.parse_expression(Precedence::LOWEST) {
            Ok(expression) => Ok(Statement::Expression(expression)),
            Err(error) => Err(error),
        };

        if self.peek_token_is(&TokenType::SEMICOLON) {
            self.next_token();
        }

        expression
    }

    // TODO needs fixing as _ arm adds extra error
    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, ParseError> {
        let mut left_expr = match self.current_token.token_type {
            TokenType::IDENT => self.parse_identifier(),
            TokenType::INTEGER => self.parse_integer_literal()?,
            TokenType::BANG | TokenType::MINUS => self.parse_prefix_expression()?,
            _ => {
                return Err(ParseError {
                    message: String::from("not implemented"),
                })
            }
        };

        while !self.peek_token_is(&TokenType::SEMICOLON) && precedence < self.peek_precedence() {
            match self.peek_token.token_type {
                TokenType::PLUS
                | TokenType::MINUS
                | TokenType::SLASH
                | TokenType::ASTERISK
                | TokenType::EQ
                | TokenType::NOTEQ
                | TokenType::LT
                | TokenType::GT => {
                    self.next_token();
                    left_expr = self.parse_infix_expression(Box::new(left_expr))?;
                }
                _ => {
                    return Err(ParseError {
                        message: format!("not parsable"),
                    })
                }
            }
        }

        Ok(left_expr)
    }

    fn parse_identifier(&self) -> Expression {
        Expression::Identifier(Identifier {
            token: self.current_token.to_owned(),
            value: self.current_token.literal.to_owned(),
        })
    }

    fn parse_integer_literal(&mut self) -> Result<Expression, ParseError> {
        let token = self.current_token.to_owned();

        match token.literal.parse::<i64>() {
            Ok(val) => Ok(Expression::Integer(IntegerLiteral { token, value: val })),
            Err(_val) => Err(ParseError {
                message: format!(
                    "expected INTEGER, got = {:?}",
                    self.current_token.token_type
                ),
            }),
        }
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression, ParseError> {
        let token = self.current_token.to_owned();
        let operator = self.current_token.literal.to_owned();

        self.next_token();
        let right = Box::new(self.parse_expression(Precedence::PREFIX)?);

        Ok(Expression::Prefix(PrefixExpression {
            token,
            operator,
            right,
        }))
    }

    fn parse_infix_expression(&mut self, left: Box<Expression>) -> Result<Expression, ParseError> {
        let token = self.current_token.to_owned();
        let operator = self.current_token.literal.to_owned();

        let precedence = self.current_precedence();
        self.next_token();

        Ok(Expression::Infix(InfixExpression {
            token,
            operator,
            left,
            right: Box::new(self.parse_expression(precedence)?),
        }))
    }

    fn parse_let_statement(&mut self) -> Result<Statement, ParseError> {
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

        Ok(Statement::Let(LetStatement {
            token,
            name: identifier,
        }))
    }

    fn parse_return_statement(&mut self) -> Result<Statement, ParseError> {
        let token = self.current_token.clone();

        while !self.current_token_is(&TokenType::SEMICOLON) {
            self.next_token();
        }

        Ok(Statement::Return(ReturnStatement { token }))
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
            false
        };
    }

    fn precedence_for(&self, token: &TokenType) -> Precedence {
        match token {
            TokenType::EQ | TokenType::NOTEQ => Precedence::EQUALS,
            TokenType::LT | TokenType::GT => Precedence::LESSGREATER,
            TokenType::PLUS | TokenType::MINUS => Precedence::SUM,
            TokenType::SLASH | TokenType::ASTERISK => Precedence::PRODUCT,
            _ => Precedence::LOWEST,
        }
    }

    fn peek_precedence(&self) -> Precedence {
        self.precedence_for(&self.peek_token.token_type)
    }

    fn current_precedence(&self) -> Precedence {
        self.precedence_for(&self.current_token.token_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Display;

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
        assert_eq!(program.to_string(), "let x = ;let y = ;let foobar = ;");
    }

    #[test]
    fn test_invalid_statements() {
        let input = r#"
let x 5;
let = 10;
let 838383;
"#;
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        parser.parse_program();
        check_parser_errors(&parser);

        // TODO needs fixing
        assert_eq!(parser.errors.len(), 4);
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
            assert_eq!("return ;".to_owned(), s.to_string());
        }
    }

    #[test]
    fn test_identifier_expression() {
        let input = r#"foobar;"#;
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        check_parser_errors(&parser);

        assert_eq!(1, program.statements.len());

        for s in program.statements {
            assert_eq!(s.to_string(), "foobar");
        }
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = r#"5;"#;
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        check_parser_errors(&parser);

        for s in program.statements {
            assert_eq!(s.to_string(), "5");
        }
    }

    #[test]
    fn test_prefix_expression() {
        let input = r#"!5;"#;
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        check_parser_errors(&parser);

        assert_eq!(1, program.statements.len());

        for s in program.statements {
            assert_eq!(s.to_string(), "(!5)");
        }
    }

    #[test]
    fn test_infix_expression() {
        let tests = vec![
            ("5 + 5;", 5, "+", 5),
            ("5 - 5;", 5, "-", 5),
            ("5 * 5;", 5, "*", 5),
            ("5 / 5;", 5, "/", 5),
            ("5 > 5;", 5, ">", 5),
            ("5 < 5;", 5, "<", 5),
            ("5 == 5;", 5, "==", 5),
            ("5 != 5;", 5, "!=", 5),
        ];

        for (input, left, operator, right) in tests {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            let operator = operator.to_owned();

            let program = parser.parse_program();
            check_parser_errors(&parser);

            assert_eq!(parser.errors.len(), 0);
            assert_eq!(
                program.statements,
                vec![Box::new(Statement::Expression(Expression::Infix(
                    InfixExpression {
                        token: Token::new(
                            operator.parse::<TokenType>().unwrap(),
                            operator.to_owned()
                        ),
                        left: Box::new(Expression::Integer(IntegerLiteral {
                            token: Token::new(TokenType::INTEGER, left.to_string()),
                            value: left,
                        })),
                        operator,
                        right: Box::new(Expression::Integer(IntegerLiteral {
                            token: Token::new(TokenType::INTEGER, right.to_string()),
                            value: right,
                        }))
                    }
                )))]
            );
        }
    }

    #[test]
    fn test_operator_precedence_parsing() {
        let tests = [
            ("-a * b", "((-a) * b)"),
            ("!-a", "(!(-a))"),
            ("a + b + c", "((a + b) + c)"),
            ("a + b - c", "((a + b) - c)"),
            ("a * b * c", "((a * b) * c)"),
            ("a * b / c", "((a * b) / c)"),
            ("a + b / c", "(a + (b / c))"),
            ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
            ("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
            ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
            ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
            (
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            ),
        ];

        for (input, expected) in tests.iter() {
            let l = Lexer::new(input);
            let mut parser = Parser::new(l);
            let program = parser.parse_program();
            check_parser_errors(&parser);

            assert_eq!(parser.errors.len(), 0);
            assert_eq!(format!("{}", program), *expected);
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
