use super::token;
use super::token::{Token, TokenType};

struct Lexer<'l> {
    input: &'l [u8],
    position: usize,
    read_position: usize,
    ch: u8,
}

impl<'l> Lexer<'l> {
    fn new(input: &'l str) -> Self {
        let mut lexer = Lexer {
            input: input.as_bytes(),
            position: 0,
            read_position: 0,
            ch: b'0',
        };

        lexer.read_char();

        lexer
    }

    fn read_char(&mut self) {
        match self.input.get(self.read_position) {
            Some(ch) => self.ch = *ch,
            None => self.ch = 0,
        }

        self.position = self.read_position;
        self.read_position = self.read_position + 1;
    }

    fn token(&mut self) -> Result<Token, failure::Error> {
        self.skip_whitespace();

        match self.ch {
            _ => {
                let literal = self.read_identifier()?;
                let token_type: TokenType = token::keyword(literal.as_str())?;

                let token = Token {
                    literal: literal,
                    token_type: token_type,
                };

                return Ok(token);
            }
        };
    }

    fn read_identifier(&mut self) -> Result<String, failure::Error> {
        let position = self.position;

        while is_letter(self.ch) {
            self.read_char()
        }

        match String::from_utf8(self.input[position..self.position].to_vec()) {
            Ok(ident) => Ok(ident),
            Err(err) => Err(failure::Error::from(err)),
        }
    }

    fn skip_whitespace(&mut self) {
        while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {
            self.read_char()
        }
    }
}

fn is_letter(ch: u8) -> bool {
    b'a' <= ch && ch <= b'z' || b'A' <= ch && ch <= b'Z' || ch == b'_'
}
#[test]
fn test_read_char() {
    let mut l = Lexer::new("abcde");
    l.read_position = 4;
    l.read_char();

    assert_eq!(l.ch, b'e')
}
#[test]
fn test_token() {
    let mut l = Lexer::new("create");

    assert_eq!(l.token().unwrap().literal, "create");
}
