#[derive(Debug, PartialEq)]
pub enum Token {
    //Special Tokens
    Eof,
    Illegal,

    //Identifiers
    Identifier(String),
    Int(String),
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

pub fn lookup_ident(ident: &str) -> Token {
    match ident {
        "fn" => Token::Fn,
        "let" => Token::Let,
        "true" => Token::True,
        "false" => Token::False,
        "if" => Token::If,
        "else" => Token::Else,
        "and" => Token::And,
        "or" => Token::Or,
        "return" => Token::Return,
        _ => Token::Identifier(ident.to_string()),
    }
}