use crate::lexer::Lexer;
use crate::token::TokenType;
use std::io;
use std::io::prelude::*;

const PROMPT: &'static str = ">> ";

// TIL
// https://stackoverflow.com/questions/39464237/whats-the-idiomatic-way-to-reference-bufreader-bufwriter-when-passing-it-between/39464443#39464443
pub fn start<R: io::Read, W: io::Write>(
    input: &mut io::BufReader<R>,
    output: &mut io::BufWriter<W>,
) -> io::Result<()> {
    loop {
        write!(output, "{}", PROMPT)?;
        output.flush()?;

        // create a new buffer string
        let mut line = String::new();
        // read line from BufReader
        input.read_line(&mut line)?;

        let mut lexer = Lexer::new(&line);

        loop {
            let tok = lexer.next_token();
            if tok.token_type == TokenType::EOF {
                break;
            }
            writeln!(output, "{:?}", tok)?;
        }
    }
}
