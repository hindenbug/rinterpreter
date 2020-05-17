use crate::token::Token;
use std::fmt;
use std::path::Prefix;

pub enum Node {
    Program(Program),
    Statement(Statement),
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Node::Program(program) => program.fmt(f),
            Node::Statement(statement) => statement.fmt(f),
        }
    }
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Box<Statement>>,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for statement in &self.statements {
            statement.fmt(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(Expression),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::Let(statement) => statement.fmt(f),
            Statement::Return(statement) => statement.fmt(f),
            Statement::Expression(expression) => expression.fmt(f),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    //pub value: Expression,
}

impl fmt::Display for LetStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "let {} = ;", self.name.value)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReturnStatement {
    pub token: Token,
    //pub return_value: Expression,
}

impl fmt::Display for ReturnStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "return ;")
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Identifier(Identifier),
    Integer(IntegerLiteral),
    Prefix(PrefixExpression),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Identifier(ident) => write!(f, "{}", ident),
            Expression::Integer(val) => write!(f, "{}", val),
            Expression::Prefix(prefix) => write!(f, "({}{})", prefix.operator, prefix.right),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl fmt::Display for IntegerLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Box<Expression>,
}

impl fmt::Display for PrefixExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}{})", self.operator, self.right)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::TokenType;
    use std::fmt::Display;

    #[test]
    fn test_string() {
        let token = Token::new(TokenType::LET, "let".to_owned());
        let identifier = Identifier {
            token: Token::new(TokenType::IDENT, "myVar".to_owned()),
            value: "myVar".to_owned(),
        };
        let _value = Box::new(Identifier {
            token: Token::new(TokenType::IDENT, "anotherVar".to_owned()),
            value: "anotherVar".to_owned(),
        });
        let let_statement = Statement::Let(LetStatement {
            token,
            name: identifier,
        });

        let program = Program {
            statements: vec![Box::new(let_statement)],
        };
        assert_eq!(program.to_string(), "let myVar = ;");

        let integer_literal = IntegerLiteral {
            token: Token::new(TokenType::INTEGER, "5".to_owned()),
            value: 5,
        };
        assert_eq!(integer_literal.to_string(), "5");

        let pe = PrefixExpression {
            token: Token::new(TokenType::INTEGER, "5".to_owned()),
            operator: String::from("+"),
            right: Box::new(Expression::Integer(IntegerLiteral {
                token: Token::new(TokenType::INTEGER, "5".to_owned()),
                value: 5,
            })),
        };
        assert_eq!(pe.to_string(), "(+5)");
    }
}
