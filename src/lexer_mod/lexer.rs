use super::token;
use super::token::Token;
use std::str::Chars;
use std::iter::Peekable;

fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        Lexer { input: input.chars().peekable() }
    }

    pub fn read_char(&mut self) -> Option<char> {
        self.input.next()
    }

    pub fn peek_char(&mut self) -> Option<&char> {
        self.input.peek()
    }

    pub fn peek_char_eq(&mut self, ch: char) -> bool {
        match self.peek_char() {
            Some(&peek_ch) => peek_ch == ch,
            None => false,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.peek_char() {
            if !c.is_whitespace() {
                break;
            }
            self.read_char();
        }
    }

    fn peek_is_letter(&mut self) -> bool {
        match self.peek_char() {
            Some(&ch) => is_letter(ch),
            None => false,
        }
    }

    fn read_identifier(&mut self, first: char) -> String {
        let mut ident = String::new();
        ident.push(first);

        while self.peek_is_letter() {
            ident.push(self.read_char().unwrap());
        }
        ident
    }

    fn read_number(&mut self, first: char) -> String {
        let mut number = String::new();
        number.push(first);

        while let Some(&c) = self.peek_char() {
            if !c.is_numeric() {
                break;
            }
            number.push(self.read_char().unwrap());
        }
        number
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        match self.read_char() {
            Some('=') => {
                if self.peek_char_eq('=') {
                    self.read_char();
                    Token::Equal
                } else {
                    Token::Assign
                }
            }
            Some('!') => {
                if self.peek_char_eq('=') {
                    self.read_char();
                    Token::NotEqual
                } else {
                    Token::Exclamation
                }
            }
            Some('+') => {
                if self.peek_char_eq('+') {
                    self.read_char();
                    Token::Increment
                } else {
                    Token::Plus
                }
            }
            Some('-') => {
                if self.peek_char_eq('-') {
                    self.read_char();
                    Token::Decrement
                } else {
                    Token::Minus
                }
            }
            Some('>') => {
                if self.peek_char_eq('=') {
                    self.read_char();
                    Token::MoreThanAndEqual
                } else {
                    Token::MoreThan
                }
            }
            Some('<') => {
                if self.peek_char_eq('=') {
                    self.read_char();
                    Token::LessThanAndEqual
                } else {
                    Token::LessThan
                }
            }

            Some('/') => {
                Token::Slash
            }
            Some('*') => {
                Token::Asterisk
            }
            Some(';') => {
                Token::Semicolon
            }
            Some(',') => {
                Token::Comma
            }
            Some('(') => {
                Token::LeftParanthesis
            }
            Some(')') => {
                Token::RightParanthesis
            }
            Some('{') => {
                Token::LeftBrace
            }
            Some('}') => {
                Token::RightBrace
            }
            Some(ch @ _) => {
                if is_letter(ch) {
                    let literal = self.read_identifier(ch);
                    token::lookup_ident(&literal)
                } else if ch.is_numeric() {
                    Token::Int(self.read_number(ch))
                } else {
                    Token::Illegal
                }
            }

            None => Token::Eof
        }
    }

}
