use super::ParseItem;
use crate::lexer_mod::lexer;
use crate::lexer_mod::lexer::Lexer;
use crate::lexer_mod::token::Token;
use std::mem;

pub enum Order {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
    Index,
}

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

    fn token_order (token: &Token) -> Order {
        match token {
            Token::Equal | Token::NotEqual => Order::Equals,
            Token::LessThan | Token::LessThanAndEqual | Token::MoreThan | Token::MoreThanAndEqual => Order::LessGreater,
            Token::Plus | Token::Minus => Order::Sum,
            Token::Asterisk | Token::Slash => Order::Product,
            Token::LeftBrace => Order::Index,
            Token::LeftParanthesis => Order::Call,
            _ => Order::Lowest,
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

    fn parse_identifier(&mut self) -> Option<String> {
        match *self.current_token {
            Token::Identifier(ident) => Some(ident),
            _ => None
        }
    }

    pub fn parse_statement(&mut self) -> Option<ParseItem::Statement> {
        match(*self.current_token) {
            Token::Let => self.parse_let_statement(),
            _ => None
        }
    }

    fn parse_expression(&mut self, order: Order) -> Option<ParseItem::Expression> {
        let mut left = match *self.current_token {
            Token::Identifier(_) => self.parse_identifier_expression(),
            Token::And | Token::False => self.parse_bool_expression(),
            Token::Int(_) => self.parse_int_expression(),
            Token::LeftBrace => self.parse_array_expression(),
            Token::Fn => self.parse_function_expression(),
            // Token::If => self.parse_if_expression(), //
            _ => {
                self.error_prefix_parser();
                return None;
            }
         }

        while !self.expect_next_token(Token::Semicolon) && order < self.next_token_order() {
            match *self.next_token {
                Token::Plus | Token::Minus
                    | Token::Slash | Token::Asterisk
                    | Token::Equal | Token::NotEqual
                    | Token::LessThan | Token::LessThanAndEqual
                    | Token::MoreThan | Token::MoreThanAndEqual => {
                        self.next_token();
                        left = self.parse_infix_expression(left.unwrap());
                    }
                Token::LeftBrace => {
                    self.next_token();
                    left = self.parse_index_expression(left.unwrap());
                }
                Token::LeftParanthesis => {
                    self.next_token();
                    left = parse_call_expression(left.unwrap());
                }
                _ => return left,
            }
        }

        left
    }

    // fn parse_index_expression(&mut self, left: ParseItem::Expression) -> Option<ParseItem::Expression> {
    //     self.next_token();
    //     let index = match self.parse_expression(Order::Lowest) {
    //         Some(expr) => expr,
    //         None => return None,
    //     };

    //     if !self.expect_next_token(Token::RightBrace) {
    //         return None;
    //     }

    //     Some(ParseItem::Expression())
    // }

    fn parse_identifier_expression(&mut self) -> Option<ParseItem::Expression> {
        match self.parse_identifier() {
            Some(ident) => Some(ParseItem::Expression::Identifier(ident)),
            _ => return None;
        }
    }

    fn parse_bool_expression(&mut self) -> Option<ParseItem::Expression> {
        match *self.current_token {
            Token::True => Some(ParseItem::Expression(Bool(true))),
            Token::False => Some(ParseItem::Expression(Bool(false))),
            _ => return None;
        }
    }

    fn parse_int_expression(&mut self) -> Option<ParseItem::Expression> {
        match *self.current_token {
            Token::Int(ref mut int) => Some(ParseItem::Expression(Integer(int))),
            _ => return None;
        }
    }

    fn parse_expression_list(&mut self, end: Token) -> Option<Vec<ParseItem::Expression>> {
        let mut vec = vec![];

        if self.expect_next_token(end) {
            return Some(vec);
        }

        match self.parse_expression(Order::Lowest) {
            Some(expr) => vec.push(expr),
            _ => return None;
        }

        while self.exoect_next_token(Token::Comma) {
            self.next_token();
            self.next_token();

            match self.parse_expression(Order::Lowest) {
                Some(expr) => vec.push(expr),
                _ => return None;
            }
        }

        if !self.expect_next_token(end) {
            return None;
        }
        Some(vec)

    }

    fn parse_array_expression(&mut self) -> Option<ParseItem::Expression> {
        match self.parse_expression_list(Token::RightBrace) {
            Some(ls) => Some(ParseItem::Expression(Array(ls))),
            _ => return None;
        }
    }

    fn parse_function_parameters(&mut self) -> Option<Vec<String>> {
        let mut params = vec![];

        if self.expect_next_token(Token::RightParanthesis) {
            self.next_token();
            return Some(params);
        }

        self.next_token();

        match self.parse_identifier() {
            Some(ident) => params.push(ident),
            _ => return None;
        };

        while self.expect_next_token(Token::Comma) {
            self.next_token();
            self.next_token();

            match self.parse_identifier() {
                Some(ident) = params.push(ident),
                _ => return None;
            };
        }

        if !self.expect_next_token(Token::RightParanthesis) {
            return None;
        }

        Some(params)
    }

    fn parse_function_expression(&mut self) -> Option<ParseItem::Expression> {
        if !self.expect_next_token(Token::LeftParanthesis) {
            return None;
        }

        let params = match self.parse_function_parameters() {
            Some(params) => params,
            _ => return None;
        };

        if !self.expect_next_token(Token::LeftBrace) {
            return None;
        }

        Some(ParseItem::Expression(params, self.parse_block_statements()))
    }

    fn parse_block_statements(&mut self) -> Vec<Statement> {
        self.next_token();

        let mut vec = vec![];

        while *self.current_token != Token::RightBrace && *self.current_token != Token::Eof {
            match self.parse_statement() {
                Some(stmt) => vec.push(stmt),
                _ => {}
            }
            self.next_token();
        }

        vec
    }

    pub fn parse_let_statement(&mut self) -> Option<ParseItem::Statement> {

        match *self.next_token {
            Token::Identifier(_) => self.next_token(),
            _ => return None,
        }
        
        // let current_box = mem::replace(&mut self.current_token, Box::new(Token::Illegal));
        // let _token = *current_box;

        let ident = match self.parse_identifier() {
            Some(name) => name,
            None => return None,
        };

        if !self.expect_next_token(Token::Assign) {
            return None;
        }

        let eval = match self.parse_expression(Order::Lowest) {
            Some(expr) => expr,
            _ => return None;
        };

        while (*self.current_token) != Token::Semicolon {
            self.next_token();
        }

        Some(ParseItem::Statement::Let(ident, eval))
    }
}
