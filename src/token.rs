use std::str::FromStr;

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

#[derive(Debug)]
pub struct ParseTokenError;

impl FromStr for TokenType {
    type Err = ParseTokenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "=" => Ok(TokenType::ASSIGN),
            "+" => Ok(TokenType::PLUS),
            "," => Ok(TokenType::COMMA),
            ";" => Ok(TokenType::SEMICOLON),
            "(" => Ok(TokenType::LEFTPAREN),
            ")" => Ok(TokenType::RIGHTPAREN),
            "{" => Ok(TokenType::LEFTBRACE),
            "}" => Ok(TokenType::RIGHTBRACE),
            "fn" => Ok(TokenType::FUNCTION),
            "let" => Ok(TokenType::LET),
            _ => Err(ParseTokenError),
        }
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

    #[test]
    fn parse_test() {
        let tests = vec![("=", TokenType::ASSIGN),
                         ("+", TokenType::PLUS),
                         (",", TokenType::COMMA),
                         (";", TokenType::SEMICOLON),
                         ("(", TokenType::LEFTPAREN),
                         (")", TokenType::RIGHTPAREN),
                         ("{", TokenType::LEFTBRACE),
                         ("}", TokenType::RIGHTBRACE),
                         ("fn", TokenType::FUNCTION),
                         ("let", TokenType::LET)];

        for (s, e) in tests {
            assert_eq!(s.parse::<TokenType>().unwrap(), e);
        }
    }
}
