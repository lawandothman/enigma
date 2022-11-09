use crate::{
    ast::{Program, Statement},
    lexer::Lexer,
    token::Token,
};

pub struct Parser {
    lexer: Lexer,
    pub errors: Vec<String>,
    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut p = Self {
            lexer,
            errors: vec![],
            cur_token: Token::Illegal,
            peek_token: Token::Illegal,
        };

        // Read two tokens, so cur_token and peek_token are both set
        p.next_token();
        p.next_token();
        p
    }
    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program { statements: vec![] };

        while self.cur_token != Token::Eof {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            }
            self.next_token();
        }
        program
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token {
            Token::Let => self.parse_let_statement(),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        let name;
        match self.peek_token.clone() {
            Token::Ident(ident) => {
                self.next_token();
                name = ident;
            }
            _ => {
                self.peek_error(&Token::Ident("identifier".to_string()));
                return None;
            }
        }

        if !self.expect_peek(Token::Assign) {
            return None;
        }

        while self.cur_token != Token::Semicolon {
            self.next_token();
        }

        Some(Statement::Let(name))
    }

    fn expect_peek(&mut self, t: Token) -> bool {
        match self.peek_token == t {
            true => {
                self.next_token();
                return true;
            }
            false => {
                self.peek_error(&t);
                return false;
            }
        }
    }

    fn peek_error(&mut self, t: &Token) {
        println!("Error error");
        let msg = format!(
            "expected next token to be {}, got {} instead",
            t, self.peek_token
        );
        self.errors.push(msg);
    }
}
