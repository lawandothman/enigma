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
    Minus,
    Bang,
    Asterisk,
    Slash,

    Lt,
    Gt,

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

pub fn lookup_ident(ident: &str) -> TokenKind {
    keyword_to_token_kind(ident).unwrap_or(TokenKind::Ident)
}

fn keyword_to_token_kind(keyword: &str) -> Option<TokenKind> {
    match keyword {
        "fn" => Some(TokenKind::Function),
        "let" => Some(TokenKind::Let),
        _ => None,
    }
}
