#[derive(Debug, PartialEq)]
pub enum Token {
    //Special Tokens
    Eof,
    Illegal,

    //Identifiers
    Identifier(String),
    Int(String),
    //Operators
    Equal,
    NotEqual,
    LessThan,
    MoreThan,
    Plus,
    Minus,
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
}

impl Default for Token {
    fn default() -> Token {
        Token::Ilegal
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
        "return" => Token::Return,
    }
}
