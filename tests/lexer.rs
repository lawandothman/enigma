extern crate enigma;

#[cfg(test)]
mod lexer_tests {
    use enigma::{lexer::Lexer, token::TokenKind};

    #[test]
    fn next_token() {
        let input = "=+(){},;";

        let tests = [
            (TokenKind::Assign, "="),
            (TokenKind::Plus, "+"),
            (TokenKind::Lparen, "("),
            (TokenKind::Rparen, ")"),
            (TokenKind::Lbrace, "{"),
            (TokenKind::Rbrace, "}"),
            (TokenKind::Comma, ","),
            (TokenKind::Semicolon, ";"),
        ];

        let mut lexer = Lexer::new(input);

        for (i, (kind, literal)) in tests.iter().enumerate() {
            let tok = lexer.next_token();
            assert_eq!(
                &tok.kind, kind,
                "tests[{}] - tokentype wrong. expected={:?}, got={:?}",
                i, kind, tok.kind
            );
            assert_eq!(
                &tok.literal, literal,
                "tests[{}] - literal wrong. expected={}, got={}",
                i, literal, tok.literal
            )
        }
    }
}
