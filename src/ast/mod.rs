pub enum Node {
    Program(Program),
    Statement(Statement),
}
#[derive(Debug, PartialEq, Eq)]
pub enum Statement {
    Let(String),
    Return,
}
pub struct Program {
    pub statements: Vec<Statement>,
}
