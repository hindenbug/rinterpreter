use super::token;
use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            input: input.chars().peekable(),
        }
    }

    pub fn read_char(&mut self) -> Option<char> {
        self.input.next()
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.input.peek()
    }

    pub fn next_token(&mut self) -> token::Token {
        self.skip_whitespace();

        if let Some(c) = self.read_char() {
            let token = match c {
                '=' => {
                    if let Some('=') = self.peek_char() {
                        self.read_char();
                        token::Token::new(token::TokenType::EQ, "==".to_string())
                    } else {
                        token::Token::new(token::TokenType::ASSIGN, c.to_string())
                    }
                }
                '!' => {
                    if let Some('=') = self.peek_char() {
                        self.read_char();
                        token::Token::new(token::TokenType::NOTEQ, "!=".to_string())
                    } else {
                        token::Token::new(token::TokenType::BANG, c.to_string())
                    }
                }
                '+' | '-' | ';' | '(' | ')' | ',' | '{' | '}' | '/' | '*' | '<' | '>' => {
                    let literal = c.to_string();
                    let token_type = literal
                        .parse::<token::TokenType>()
                        .unwrap_or(token::TokenType::EOF);
                    println!("{:?}", token_type);

                    token::Token::new(token_type, literal)
                }
                _ => {
                    if Self::is_letter(c) {
                        let literal = self.read_identifier(c);
                        let token_type = token::lookup_identifier(&literal.as_str());
                        token::Token::new(token_type, literal)
                    } else if c.is_digit(10) {
                        token::Token::new(token::TokenType::INTEGER, self.read_number(c))
                    } else {
                        token::Token::new(token::TokenType::ILLEGAL, c.to_string())
                    }
                }
            };
            return token;
        } else {
            token::Token::new(token::TokenType::EOF, "".to_string())
        }
    }

    pub fn read_identifier(&mut self, c: char) -> String {
        let mut ident = String::new();
        ident.push(c);
        while let Some(&c) = self.peek_char() {
            if Self::is_letter(c) {
                ident.push(self.read_char().unwrap());
            } else {
                break;
            }
        }
        ident
    }

    pub fn read_number(&mut self, c: char) -> String {
        let mut number = String::new();
        number.push(c);
        while let Some(&c) = self.peek_char() {
            if c.is_digit(10) {
                number.push(self.read_char().unwrap());
            } else {
                break;
            }
        }
        number
    }

    fn is_letter(c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(&c) = self.peek_char() {
            if c.is_whitespace() {
                let _ = self.read_char();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = r#"let five = 5;
let ten = 10;
let add = fn(x, y) {
  x + y;
};
let result = add(five, ten);
!-/*5;
5 < 10 > 5;
if (5 < 10) {
  return true;
} else {
  return false;
}
10 == 10;
10 != 9;
"#;
        let mut tests = Vec::new();
        tests.push(token::Token::new(token::TokenType::LET, "let".to_string()));
        tests.push(token::Token::new(
            token::TokenType::IDENT,
            "five".to_string(),
        ));
        tests.push(token::Token::new(token::TokenType::ASSIGN, "=".to_string()));
        tests.push(token::Token::new(
            token::TokenType::INTEGER,
            "5".to_string(),
        ));
        tests.push(token::Token::new(
            token::TokenType::SEMICOLON,
            ";".to_string(),
        ));
        tests.push(token::Token::new(token::TokenType::LET, "let".to_string()));
        tests.push(token::Token::new(
            token::TokenType::IDENT,
            "ten".to_string(),
        ));
        tests.push(token::Token::new(token::TokenType::ASSIGN, "=".to_string()));
        tests.push(token::Token::new(
            token::TokenType::INTEGER,
            "10".to_string(),
        ));
        tests.push(token::Token::new(
            token::TokenType::SEMICOLON,
            ";".to_string(),
        ));
        tests.push(token::Token::new(token::TokenType::LET, "let".to_string()));
        tests.push(token::Token::new(
            token::TokenType::IDENT,
            "add".to_string(),
        ));
        tests.push(token::Token::new(token::TokenType::ASSIGN, "=".to_string()));
        tests.push(token::Token::new(
            token::TokenType::FUNCTION,
            "fn".to_string(),
        ));
        tests.push(token::Token::new(
            token::TokenType::LEFTPAREN,
            "(".to_string(),
        ));
        tests.push(token::Token::new(token::TokenType::IDENT, "x".to_string()));
        tests.push(token::Token::new(token::TokenType::COMMA, ",".to_string()));
        tests.push(token::Token::new(token::TokenType::IDENT, "y".to_string()));
        tests.push(token::Token::new(
            token::TokenType::RIGHTPAREN,
            ")".to_string(),
        ));
        tests.push(token::Token::new(
            token::TokenType::LEFTBRACE,
            "{".to_string(),
        ));
        tests.push(token::Token::new(token::TokenType::IDENT, "x".to_string()));
        tests.push(token::Token::new(token::TokenType::PLUS, "+".to_string()));
        tests.push(token::Token::new(token::TokenType::IDENT, "y".to_string()));
        tests.push(token::Token::new(
            token::TokenType::SEMICOLON,
            ";".to_string(),
        ));
        tests.push(token::Token::new(
            token::TokenType::RIGHTBRACE,
            "}".to_string(),
        ));
        tests.push(token::Token::new(
            token::TokenType::SEMICOLON,
            ";".to_string(),
        ));
        tests.push(token::Token::new(token::TokenType::LET, "let".to_string()));
        tests.push(token::Token::new(
            token::TokenType::IDENT,
            "result".to_string(),
        ));
        tests.push(token::Token::new(token::TokenType::ASSIGN, "=".to_string()));
        tests.push(token::Token::new(
            token::TokenType::IDENT,
            "add".to_string(),
        ));
        tests.push(token::Token::new(
            token::TokenType::LEFTPAREN,
            "(".to_string(),
        ));
        tests.push(token::Token::new(
            token::TokenType::IDENT,
            "five".to_string(),
        ));
        tests.push(token::Token::new(token::TokenType::COMMA, ",".to_string()));
        tests.push(token::Token::new(
            token::TokenType::IDENT,
            "ten".to_string(),
        ));
        tests.push(token::Token::new(
            token::TokenType::RIGHTPAREN,
            ")".to_string(),
        ));
        tests.push(token::Token::new(
            token::TokenType::SEMICOLON,
            ";".to_string(),
        ));
        tests.push(token::Token::new(token::TokenType::BANG, "!".to_string()));
        tests.push(token::Token::new(token::TokenType::MINUS, "-".to_string()));
        tests.push(token::Token::new(token::TokenType::SLASH, "/".to_string()));
        tests.push(token::Token::new(
            token::TokenType::ASTERISK,
            "*".to_string(),
        ));
        tests.push(token::Token::new(
            token::TokenType::INTEGER,
            "5".to_string(),
        ));
        tests.push(token::Token::new(
            token::TokenType::SEMICOLON,
            ";".to_string(),
        ));
        tests.push(token::Token::new(
            token::TokenType::INTEGER,
            "5".to_string(),
        ));
        tests.push(token::Token::new(token::TokenType::LT, "<".to_string()));
        tests.push(token::Token::new(
            token::TokenType::INTEGER,
            "10".to_string(),
        ));
        tests.push(token::Token::new(token::TokenType::GT, ">".to_string()));
        tests.push(token::Token::new(
            token::TokenType::INTEGER,
            "5".to_string(),
        ));
        tests.push(token::Token::new(
            token::TokenType::SEMICOLON,
            ";".to_string(),
        ));
        tests.push(token::Token::new(token::TokenType::IF, "if".to_string()));
        tests.push(token::Token::new(
            token::TokenType::LEFTPAREN,
            "(".to_string(),
        ));
        tests.push(token::Token::new(
            token::TokenType::INTEGER,
            "5".to_string(),
        ));
        tests.push(token::Token::new(token::TokenType::LT, "<".to_string()));
        tests.push(token::Token::new(
            token::TokenType::INTEGER,
            "10".to_string(),
        ));
        tests.push(token::Token::new(
            token::TokenType::RIGHTPAREN,
            ")".to_string(),
        ));
        tests.push(token::Token::new(
            token::TokenType::LEFTBRACE,
            "{".to_string(),
        ));
        tests.push(token::Token::new(
            token::TokenType::RETURN,
            "return".to_string(),
        ));
        tests.push(token::Token::new(
            token::TokenType::TRUE,
            "true".to_string(),
        ));
        tests.push(token::Token::new(
            token::TokenType::SEMICOLON,
            ";".to_string(),
        ));

        tests.push(token::Token::new(
            token::TokenType::RIGHTBRACE,
            "}".to_string(),
        ));
        tests.push(token::Token::new(
            token::TokenType::ELSE,
            "else".to_string(),
        ));
        tests.push(token::Token::new(
            token::TokenType::LEFTBRACE,
            "{".to_string(),
        ));
        tests.push(token::Token::new(
            token::TokenType::RETURN,
            "return".to_string(),
        ));
        tests.push(token::Token::new(
            token::TokenType::FALSE,
            "false".to_string(),
        ));
        tests.push(token::Token::new(
            token::TokenType::SEMICOLON,
            ";".to_string(),
        ));
        tests.push(token::Token::new(
            token::TokenType::RIGHTBRACE,
            "}".to_string(),
        ));
        tests.push(token::Token::new(
            token::TokenType::INTEGER,
            "10".to_string(),
        ));
        tests.push(token::Token::new(token::TokenType::EQ, "==".to_string()));
        tests.push(token::Token::new(
            token::TokenType::INTEGER,
            "10".to_string(),
        ));
        tests.push(token::Token::new(
            token::TokenType::SEMICOLON,
            ";".to_string(),
        ));
        tests.push(token::Token::new(
            token::TokenType::INTEGER,
            "10".to_string(),
        ));
        tests.push(token::Token::new(token::TokenType::NOTEQ, "!=".to_string()));
        tests.push(token::Token::new(
            token::TokenType::INTEGER,
            "9".to_string(),
        ));
        tests.push(token::Token::new(
            token::TokenType::SEMICOLON,
            ";".to_string(),
        ));

        let mut lexer = Lexer::new(&input);

        for test in tests {
            let token = &lexer.next_token();
            assert_eq!(token.literal, test.literal);
            assert_eq!(test.token_type, token.token_type);
        }
    }
}
