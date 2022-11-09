pub enum Node {
    Program(Program),
    Statement(Statement),
    Expression(Expression),
}
#[derive(Debug, PartialEq, Eq)]
pub enum Statement {
    Let(String),
}

pub enum Expression {
    Identifier(String),
}

pub struct Program {
    pub statements: Vec<Statement>,
}
