use crate::token::{self, Token};

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
            '=' => match self.peek_char() {
                '=' => {
                    self.read_char();
                    Token::Eq
                }
                _ => Token::Assign,
            },
            '+' => Token::Plus,
            '-' => Token::Minus,
            '!' => match self.peek_char() {
                '=' => {
                    self.read_char();
                    Token::NotEq
                }
                _ => Token::Bang,
            },
            '/' => Token::Slash,
            '*' => Token::Asterisk,
            '<' => Token::Lt,
            '>' => Token::Gt,
            '(' => Token::Lparen,
            ')' => Token::Rparen,
            ',' => Token::Comma,
            '{' => Token::Lbrace,
            '}' => Token::Rbrace,
            ';' => Token::Semicolon,
            '\u{0}' => Token::Eof,
            _ => {
                if is_letter(self.ch) {
                    let literal = self.read_identifier();
                    return token::lookup_ident(literal);
                } else if is_digit(self.ch) {
                    return Token::Int(self.read_number().to_owned());
                } else {
                    Token::Illegal
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

    fn peek_char(&self) -> char {
        self.input
            .chars()
            .nth(self.read_position)
            .unwrap_or('\u{0}')
    }
}

fn is_letter(ch: char) -> bool {
    ('a'..='z').contains(&ch) || ('a'..='z').contains(&ch) || ch == '_'
}

fn is_digit(ch: char) -> bool {
    ('0'..='9').contains(&ch)
}
