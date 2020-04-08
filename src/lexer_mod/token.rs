use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Token {
    //Special Tokens
    Eof,
    Illegal,

    //Identifiers
    Identifier(String),
    Int(i64),
    //Operators
    Assign,
    Equal,
    NotEqual,
    LessThan,
    LessThanAndEqual,
    MoreThan,
    MoreThanAndEqual,
    Plus,
    Increment,
    Minus,
    Decrement,
    Asterisk,
    Slash,
    Exclamation,

    //Delimiters
    Comma,
    Semicolon,
    LeftParanthesis,
    RightParanthesis,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,

    //Keywords
    Let,
    Fn,
    Extern,
    True,
    False,
    If,
    Else,
    Return,
    And,
    Or,
}

impl Default for Token {
    fn default() -> Token {
        Token::Illegal
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Slash => write!(f, "/"),
            Token::Asterisk => write!(f, "*"),
            Token::Equal => write!(f, "=="),
            Token::NotEqual => write!(f, "!="),
            Token::MoreThanAndEqual => write!(f, ">="),
            Token::MoreThan => write!(f, ">"),
            Token::LessThanAndEqual => write!(f, "<="),
            Token::LessThan => write!(f, "<"),
            Token::Assign => write!(f, "="),
            Token::Comma => write!(f, ","),
            Token::Semicolon => write!(f, ";"),
            Token::LeftParanthesis => write!(f, "("),
            Token::RightParanthesis => write!(f, ")"),
            Token::LeftBrace => write!(f, "{{"),
            Token::RightBrace => write!(f, "}}"),
            Token::LeftBracket => write!(f, "["),
            Token::RightBracket => write!(f, "]"),
            Token::Exclamation => write!(f, "!"),
            Token::Increment => write!(f, "++"),
            Token::Decrement => write!(f, "--"),
            _ => write!(f, "{:?}", self)
        }
    }
}

pub fn get_identifier(idnt: &str) -> Token {
    match idnt {
        "fn" => Token::Fn,
        "let" => Token::Let,
        "true" => Token::True,
        "false" => Token::False,
        "if" => Token::If,
        "else" => Token::Else,
        "and" => Token::And,
        "or" => Token::Or,
        "return" => Token::Return,
        _ => Token::Identifier(idnt.to_string()),
    }
}
