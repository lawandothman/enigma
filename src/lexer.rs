use crate::token::{self, Token, TokenKind};

pub struct Lexer {
    input: String,
    // current position in input (points to current char)
    position: usize,
    // current reading position in input (after current char)
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer {
            input: input.to_string(),
            position: 0,
            read_position: 0,
            ch: '\u{0}',
        };
        lexer.read_char();
        lexer
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let tok = match self.ch {
            '=' => new_token(TokenKind::Assign, self.ch),
            ';' => new_token(TokenKind::Semicolon, self.ch),
            '(' => new_token(TokenKind::Lparen, self.ch),
            ')' => new_token(TokenKind::Rparen, self.ch),
            ',' => new_token(TokenKind::Comma, self.ch),
            '+' => new_token(TokenKind::Plus, self.ch),
            '{' => new_token(TokenKind::Lbrace, self.ch),
            '}' => new_token(TokenKind::Rbrace, self.ch),
            '\u{0}' => Token {
                kind: TokenKind::Eof,
                literal: "".to_owned(),
            },
            _ => {
                if is_letter(self.ch) {
                    let literal = self.read_identifier();
                    let kind = token::lookup_ident(literal);
                    return Token {
                        kind,
                        literal: literal.to_owned(),
                    };
                } else if is_digit(self.ch) {
                    return Token {
                        kind: TokenKind::Int,
                        literal: self.read_number().to_owned(),
                    };
                } else {
                    Token {
                        kind: TokenKind::Illegal,
                        literal: "".to_owned(),
                    }
                }
            }
        };
        self.read_char();
        tok
    }

    fn read_char(&mut self) {
        self.ch = self
            .input
            .chars()
            .nth(self.read_position)
            .unwrap_or('\u{0}');
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> &str {
        let position = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }
        &self.input[position..self.position]
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char()
        }
    }

    fn read_number(&mut self) -> &str {
        let position = self.position;
        while is_digit(self.ch) {
            self.read_char()
        }
        &self.input[position..self.position]
    }
}
fn new_token(kind: TokenKind, ch: char) -> Token {
    Token {
        kind,
        literal: ch.to_string(),
    }
}

fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}
