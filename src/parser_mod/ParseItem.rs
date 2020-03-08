use crate::lexer_mod::token::Token;
pub type Identifier = String;

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Program {
        Program {
            statements: Vec::new(),
        }
    }
}

pub enum Statement {
    Let(Identifier, Expression),
    Return(Expression),
    Return(Expression),
    None,
}

pub enum Expression {
    Bool(bool),
    Identifier(Identifier),
    Integer(i64),
    String(String),
    Array(Vec<Expression>),
    Function(Vec<Identifier>, Vec<Statement>),
}
