use crate::token::{Token, TokenKind};

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
            _ => new_token(TokenKind::Illegal, self.ch),
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
}
fn new_token(kind: TokenKind, ch: char) -> Token {
    Token {
        kind,
        literal: ch.to_string(),
    }
}
