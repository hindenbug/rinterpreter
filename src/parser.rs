use crate::ast::Program;
use crate::lexer::Lexer;
use crate::token::Token;

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
}

impl Parser {
    fn new(lexer: Lexer) -> Self {
        Parser {
            lexer,
            current_token: lexer.next_token(),
            peek_token: lexer.next_token(),
        }
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token;
        self.peek_token = self.lexer.next_token()
    }

    pub fn parse_program(&mut self) -> Program {
        todo!();
    }
}
