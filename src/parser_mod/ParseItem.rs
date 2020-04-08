use std::fmt;
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

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Let(Identifier, Expression),
    Return(Expression),
    BlockStatement(Vec<Statement>),
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Bool(bool),
    Identifier(Identifier),
    Integer(i64),
    Call {
        func: Box<Expression>,
        args: Vec<Expression>,
    },
    Array(Vec<Expression>),
    Function(Vec<Identifier>, Box<Statement>),
    Infix(Infix, Box<Expression>, Box<Expression>),
    Prefix(Prefix, Box<Expression>),
    Index(Box<Expression>, Box<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Infix {
    Plus,
    Minus,
    Divide,
    Multiply,
    Equal,
    NotEqual,
    MoreThanAndEqual,
    MoreThan,
    LessThanAndEqual,
    LessThan
}

impl fmt::Display for Infix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Infix::Plus => write!(f, "+"),
            Infix::Minus => write!(f, "-"),
            Infix::Divide => write!(f, "/"),
            Infix::Multiply => write!(f, "*"),
            Infix::Equal => write!(f, "=="),
            Infix::NotEqual => write!(f, "!="),
            Infix::MoreThanAndEqual => write!(f, ">="),
            Infix::MoreThan => write!(f, ">"),
            Infix::LessThanAndEqual => write!(f, "<="),
            Infix::LessThan => write!(f, "<"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Prefix {
    Plus,
    Minus,
    Not
}

impl fmt::Display for Prefix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Prefix::Plus => write!(f, "+"),
            Prefix::Minus => write!(f, "-"),
            Prefix::Not => write!(f, "!"),
        }
    }
}
