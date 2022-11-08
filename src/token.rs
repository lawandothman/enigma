#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    Illegal,
    Eof,

    // Identifiers + literals
    Ident, // add, foobar, x, y
    Int,   // 1343456

    // Operators
    Assign,
    Plus,

    // Delimiters
    Comma,
    Semicolon,

    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    // Keywords
    Function,
    Let,
}
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
}
