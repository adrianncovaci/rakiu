use super::ParseItem;
use crate::lexer_mod::lexer;
use crate::lexer_mod::lexer::Lexer;
use crate::lexer_mod::token::Token;
use std::mem;

pub struct Parser<'a> {
    pub current_token: Box<Token>,
    pub next_token: Box<Token>,
    lexer: Lexer<'a>,
    err_list: Vec<Result<(), String>>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer_: Lexer<'a>) -> Parser<'a> {
        Parser {
            lexer: lexer_,
            current_token: Box::new(Token::Illegal),
            next_token: Box::new(Token::Illegal),
            err_list: Vec::new(),
        }
    }

    pub fn next_token(&mut self) {
        self.current_token = mem::replace(&mut self.next_token, Box::new(self.lexer.next_token()));
    }

    fn expect_next_token(&mut self, token: Token) -> bool {
        match *self.next_token {
            token => { self.next_token(); true }
            _ => { self.err_list.push(Err(format!("Expected {:?}, but found {:?}", token, *self.next_token))); false }
        }
    }

    pub fn parse_program(&mut self) -> ParseItem::Program {
        let mut program = ParseItem::Program::new();
        loop {
            match(*self.current_token) {
                Token::Eof => break,
                _ => {
                    let statement_parse = self.parse_statement();
                    match statement_parse {
                        Some(statement_parse) => program.statements.push(statement_parse),
                        None => (),
                    }
                    self.next_token();
                }
            }
        }
        program
    }

    pub fn parse_statement(&mut self) -> Option<ParseItem::Statement> {
        match(*self.current_token) {
            Token::Let => self.parse_let_statement(),
            _ => None
        }
    }

    pub fn parse_let_statement(&mut self) -> Option<ParseItem::Statement> {
        if !self.expect_next_token(Token::Identifier) {
            return None;
        }

        let current_box = mem::replace(&mut self.current_token, Box::new(Token::Illegal));
        let _token = *current_box;

        if !self.expect_next_token(Token::Assign) {
            return None;
        }

        let eval = Token::Identifier;

        while (*self.current_token) != Token::Semicolon {
            self.next_token();
        }

        Some(ParseItem::Statement::Let(_token, eval))
    }
}
