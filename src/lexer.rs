use super::token;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: Option<char>
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lxr = Lexer {
            input: input,
            position: 0,
            read_position: 0,
            ch: None
        };
        lxr.read_char();
        return lxr;
    }

    pub fn read_char(&mut self) {
        let chars = self.input.chars().collect::<Vec<char>>();

        if chars.len() > self.read_position {
            self.ch = Some(chars[self.read_position]);
        } else {
            self.ch = None;
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> token::Token {
        self.skip_whitespace();

        if let Some(c) = self.ch {
            let token = match c {
                '+' | '=' | '(' | ')' | '{' | '}' |
                ',' | ';'  => {
                    let literal = c.to_string();
                    let token_type = literal.parse::<token::TokenType>().unwrap_or(token::TokenType::EOF);
                    self.read_char();
                    println!("{:?}", token_type);

                    token::Token::new(token_type, literal)
                },
                _ => {
                    if self.is_letter() {
                        let literal = self.read_identifier();
                        let token_type = token::lookup_identifier(&literal.as_str());
                        token::Token::new(token_type, literal)
                    } else if self.is_digit() {
                        token::Token::new(token::TokenType::INTEGER, self.read_number())
                    } else {
                        token::Token::new(token::TokenType::ILLEGAL, c.to_string())
                    }
                }
            };
            return token;
        }
        self.read_char();
        token::Token::new(token::TokenType::EOF, "".to_string())
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;

        while self.is_letter() {
            self.read_char();
        }

        self.input
            .chars()
            .collect::<Vec<char>>()
            .get(position..self.position)
            .unwrap()
            .into_iter()
            .map(|c| c.clone())
            .collect()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;

        while self.is_digit() {
            self.read_char();
        }

        self.input
            .chars()
            .collect::<Vec<char>>()
            .get(position..self.position)
            .unwrap()
            .into_iter()
            .map(|c| c.clone())
            .collect()
    }

    fn is_digit(&mut self) -> bool {
      self.ch.unwrap().is_digit(10)
    }

    fn is_letter(&mut self) -> bool {
        self.ch.unwrap().is_alphabetic() || self.ch.unwrap() == '_'
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.ch {
            if c.is_whitespace() {
                self.read_char();
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
        let input = "let five = 5;
        let ten = 10;
        let add = fn(x, y) {
          x + y;
        };";
        //let result = add(five, ten);";

        let mut tests = Vec::new();
        tests.push(token::Token::new(token::TokenType::LET, "let".to_string()));
        tests.push(token::Token::new(token::TokenType::IDENT, "five".to_string()));
        tests.push(token::Token::new(token::TokenType::ASSIGN, "=".to_string()));
        tests.push(token::Token::new(token::TokenType::INTEGER, "5".to_string()));
        tests.push(token::Token::new(token::TokenType::SEMICOLON, ";".to_string()));
        tests.push(token::Token::new(token::TokenType::LET, "let".to_string()));
        tests.push(token::Token::new(token::TokenType::IDENT, "ten".to_string()));
        tests.push(token::Token::new(token::TokenType::ASSIGN, "=".to_string()));
        tests.push(token::Token::new(token::TokenType::INTEGER, "10".to_string()));
        tests.push(token::Token::new(token::TokenType::SEMICOLON, ";".to_string()));
        tests.push(token::Token::new(token::TokenType::LET, "let".to_string()));
        tests.push(token::Token::new(token::TokenType::IDENT, "add".to_string()));
        tests.push(token::Token::new(token::TokenType::ASSIGN, "=".to_string()));
        tests.push(token::Token::new(token::TokenType::FUNCTION, "fn".to_string()));
        tests.push(token::Token::new(token::TokenType::LEFTPAREN, "(".to_string()));
        tests.push(token::Token::new(token::TokenType::IDENT, "x".to_string()));
        tests.push(token::Token::new(token::TokenType::COMMA, ",".to_string()));
        tests.push(token::Token::new(token::TokenType::IDENT, "y".to_string()));
        tests.push(token::Token::new(token::TokenType::RIGHTPAREN, ")".to_string()));
        tests.push(token::Token::new(token::TokenType::LEFTBRACE, "{".to_string()));
        tests.push(token::Token::new(token::TokenType::IDENT, "x".to_string()));
        tests.push(token::Token::new(token::TokenType::PLUS, "+".to_string()));
        tests.push(token::Token::new(token::TokenType::IDENT, "y".to_string()));
        tests.push(token::Token::new(token::TokenType::SEMICOLON, ";".to_string()));
        tests.push(token::Token::new(token::TokenType::RIGHTBRACE, "}".to_string()));
        tests.push(token::Token::new(token::TokenType::SEMICOLON, ";".to_string()));
        // tests.push(token::Token::new(token::TokenType::LET, "let".to_string()));
        // tests.push(token::Token::new(token::TokenType::IDENT, "result".to_string()));
        // tests.push(token::Token::new(token::TokenType::ASSIGN, "=".to_string()));
        // tests.push(token::Token::new(token::TokenType::FUNCTION, "add".to_string()));
        // tests.push(token::Token::new(token::TokenType::LEFTPAREN, "(".to_string()));
        // tests.push(token::Token::new(token::TokenType::IDENT, "five".to_string()));
        // tests.push(token::Token::new(token::TokenType::COMMA, ",".to_string()));
        // tests.push(token::Token::new(token::TokenType::IDENT, "ten".to_string()));
        // tests.push(token::Token::new(token::TokenType::RIGHTPAREN, ")".to_string()));
        // tests.push(token::Token::new(token::TokenType::SEMICOLON, ";".to_string()));

        let mut lexer = Lexer::new(input.to_string());

        for test in tests {
            let token = lexer.next_token();
            assert_eq!(token.literal, test.literal);
            assert_eq!(test.token_type, token.token_type);
        }
    }
}
