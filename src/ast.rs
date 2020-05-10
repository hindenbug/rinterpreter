use crate::token::Token;

pub trait Node {
    fn token_literal(&self) -> String;
    fn to_string(&self) -> String;
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
    fn token_literal(&self) -> String {
        match self.statements.len() {
            0 => "".to_string(),
            _ => self.statements[0].token_literal(),
        }
    }

    fn to_string(&self) -> String {
        let mut s = String::new();

        for statement in &self.statements {
            s.push_str(&statement.to_string());
        }

        s
    }
}

pub struct ReturnStatement {
    pub token: Token,
    //pub return_value: Option<Box<dyn Expression>>,
}

impl Statement for ReturnStatement {
    fn statement_node(&self) {
        unimplemented!();
    }
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }

    fn to_string(&self) -> String {
        /*    let mut s = String::new();
           s += format!("{} {}", self.token_literal(), self.return_value.to_string()).as_str();
           s += ";";
           s
        */
        String::new()
    }
}

pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Option<Box<dyn Expression>>,
}

impl Statement for LetStatement {
    fn statement_node(&self) {
        unimplemented!();
    }
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }

    fn to_string(&self) -> String {
        let mut s = String::new();
        s += format!("{} {} = ", self.token_literal(), self.name.to_string()).as_str();

        if let Some(ref value) = self.value {
            s += value.to_string().as_ref();
        }

        s += ";";
        s
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
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }

    fn to_string(&self) -> String {
        self.value.to_owned()
    }
}

pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Option<Box<dyn Expression>>,
}

impl Statement for ExpressionStatement {
    fn statement_node(&self) {
        unimplemented!();
    }
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }

    fn to_string(&self) -> String {
        let mut s = String::new();
        if let Some(ref expression) = self.expression {
            s += expression.to_string().as_ref();
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::TokenType;

    #[test]
    fn test_string() {
        let token = Token::new(TokenType::LET, "let".to_owned());
        let identifier = Identifier {
            token: Token::new(TokenType::IDENT, "myVar".to_owned()),
            value: "myVar".to_owned(),
        };
        let value = Box::new(Identifier {
            token: Token::new(TokenType::IDENT, "anotherVar".to_owned()),
            value: "anotherVar".to_owned(),
        });
        let let_statement = LetStatement {
            token,
            name: identifier,
            value: Some(value),
        };

        let program = Program {
            statements: vec![Box::new(let_statement)],
        };

        assert_eq!(program.to_string(), "let myVar = anotherVar;");
    }
}
