use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    //IDENTIFIERS + LITERALS
    IDENT,
    INTEGER,

    //OPERATORS
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,
    EQ,
    NOTEQ,

    //DELIMITERS
    COMMA,
    SEMICOLON,
    LEFTPAREN,
    RIGHTPAREN,
    LEFTBRACE,
    RIGHTBRACE,

    //KEYWORDS
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Self {
        Token {
            token_type,
            literal,
        }
    }
}

pub fn lookup_identifier(identifier: &str) -> TokenType {
    match identifier {
        "fn" => TokenType::FUNCTION,
        "let" => TokenType::LET,
        "true" => TokenType::TRUE,
        "false" => TokenType::FALSE,
        "if" => TokenType::IF,
        "else" => TokenType::ELSE,
        "return" => TokenType::RETURN,
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
            "==" => Ok(TokenType::EQ),
            "+" => Ok(TokenType::PLUS),
            "-" => Ok(TokenType::MINUS),
            "!" => Ok(TokenType::BANG),
            "!=" => Ok(TokenType::NOTEQ),
            "*" => Ok(TokenType::ASTERISK),
            "/" => Ok(TokenType::SLASH),
            "<" => Ok(TokenType::LT),
            ">" => Ok(TokenType::GT),
            "," => Ok(TokenType::COMMA),
            ";" => Ok(TokenType::SEMICOLON),
            "(" => Ok(TokenType::LEFTPAREN),
            ")" => Ok(TokenType::RIGHTPAREN),
            "{" => Ok(TokenType::LEFTBRACE),
            "}" => Ok(TokenType::RIGHTBRACE),
            "fn" => Ok(TokenType::FUNCTION),
            "let" => Ok(TokenType::LET),
            "true" => Ok(TokenType::TRUE),
            "false" => Ok(TokenType::FALSE),
            "if" => Ok(TokenType::IF),
            "else" => Ok(TokenType::ELSE),
            "return" => Ok(TokenType::RETURN),
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
        let tests = vec![
            ("=", TokenType::ASSIGN),
            ("+", TokenType::PLUS),
            ("-", TokenType::MINUS),
            ("!", TokenType::BANG),
            ("*", TokenType::ASTERISK),
            ("/", TokenType::SLASH),
            ("<", TokenType::LT),
            (">", TokenType::GT),
            (",", TokenType::COMMA),
            (";", TokenType::SEMICOLON),
            ("(", TokenType::LEFTPAREN),
            (")", TokenType::RIGHTPAREN),
            ("{", TokenType::LEFTBRACE),
            ("}", TokenType::RIGHTBRACE),
            ("fn", TokenType::FUNCTION),
            ("let", TokenType::LET),
            ("true", TokenType::TRUE),
            ("false", TokenType::FALSE),
            ("if", TokenType::IF),
            ("else", TokenType::ELSE),
            ("return", TokenType::RETURN),
        ];

        for (s, e) in tests {
            assert_eq!(s.parse::<TokenType>().unwrap(), e);
        }
    }
}
