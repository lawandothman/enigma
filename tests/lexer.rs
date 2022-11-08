extern crate enigma;

#[cfg(test)]
mod lexer_tests {
    use enigma::{lexer::Lexer, token::TokenKind};

    #[test]
    fn next_token() {
        let input = "
			let five = 5;
			let ten = 10;

			let add = fn(x,y) {
				x + y;
			};

			let result = add(five, ten);
		";

        let tests = [
            (TokenKind::Let, "let"),
            (TokenKind::Ident, "five"),
            (TokenKind::Assign, "="),
            (TokenKind::Int, "5"),
            (TokenKind::Semicolon, ";"),
            (TokenKind::Let, "let"),
            (TokenKind::Ident, "ten"),
            (TokenKind::Assign, "="),
            (TokenKind::Int, "10"),
            (TokenKind::Semicolon, ";"),
            (TokenKind::Let, "let"),
            (TokenKind::Ident, "add"),
            (TokenKind::Assign, "="),
            (TokenKind::Function, "fn"),
            (TokenKind::Lparen, "("),
            (TokenKind::Ident, "x"),
            (TokenKind::Comma, ","),
            (TokenKind::Ident, "y"),
            (TokenKind::Rparen, ")"),
            (TokenKind::Lbrace, "{"),
            (TokenKind::Ident, "x"),
            (TokenKind::Plus, "+"),
            (TokenKind::Ident, "y"),
            (TokenKind::Semicolon, ";"),
            (TokenKind::Rbrace, "}"),
            (TokenKind::Semicolon, ";"),
            (TokenKind::Let, "let"),
            (TokenKind::Ident, "result"),
            (TokenKind::Assign, "="),
            (TokenKind::Ident, "add"),
            (TokenKind::Lparen, "("),
            (TokenKind::Ident, "five"),
            (TokenKind::Comma, ","),
            (TokenKind::Ident, "ten"),
            (TokenKind::Rparen, ")"),
            (TokenKind::Semicolon, ";"),
            (TokenKind::Eof, ""),
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
