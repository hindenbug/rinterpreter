#[derive(Debug, PartialEq)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    //IDENTIFIERS + LITERALS
    IDENT,
    INTEGER,

    //OPERATORS
    ASSIGN,
    PLUS,

    //DELIMITERS
    COMMA,
    SEMICOLON,
    LEFTPAREN,
    RIGHTPAREN,
    LEFTBRACE,
    RIGHTBRACE,

    //KEYWORDS
    FUNCTION,
    LET
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Self {
        Token {
            token_type: token_type,
            literal: literal
        }
    }
}

pub fn lookup_identifier(identifier: &str) -> TokenType {
    match identifier {
        "fn" => TokenType::FUNCTION,
        "let" => TokenType::LET,
        _ => TokenType::IDENT,
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookup_identifier_test() {
        assert_eq!(lookup_identifier("fn"), TokenType::FUNCTION);
        assert_eq!(lookup_identifier("assda"), TokenType::IDENT);
    }
}
