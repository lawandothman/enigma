#[cfg(test)]
mod parser_tests {
    use std::vec;

    use enigma::{ast::Statement, lexer::Lexer, parser::Parser};

    #[test]
    fn test_let_statement() {
        let input = "
			let x = 5;
			let y = 10;
			let foobar = 838383;
		";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();

        assert_eq!(
            program.statements,
            vec![
                Statement::Let("x".to_string()),
                Statement::Let("y".to_string()),
                Statement::Let("foobar".to_string())
            ]
        )
    }
}
