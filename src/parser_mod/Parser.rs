use super::ParseItem;
use crate::lexer_mod::lexer;
use crate::lexer_mod::lexer::Lexer;
use crate::lexer_mod::token::Token;

use std::fmt;
use std::mem;

pub type ParseErrors = Vec<ParseError>;
pub type Program = Vec<ParseItem::Statement>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Order {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
    Index,
    Assign,
}

#[derive(Debug, Clone)]
pub enum Error {
    UnexpectedToken,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::UnexpectedToken => write!(f, "Unexpected Token!"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParseError {
    kind: Error,
    msg: String,
}

impl ParseError {
    fn new(kind: Error, msg: String) -> Self {
        ParseError { kind, msg }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.msg)
    }
}

pub struct Parser<'a> {
    pub current_token: Box<Token>,
    pub next_token: Box<Token>,
    lexer: Lexer<'a>,
    err_list: ParseErrors,
}

impl<'a> Parser<'a> {
    pub fn new(lexer_: Lexer<'a>) -> Parser<'a> {
        let mut parser = Parser {
            lexer: lexer_,
            current_token: Box::new(Token::Illegal),
            next_token: Box::new(Token::Illegal),
            err_list: Vec::new(),
        };
        parser.next_token();
        parser.next_token();

        parser
    }

    pub fn get_errors(&mut self) -> ParseErrors {
        self.err_list.clone()
    }

    fn error_next(&mut self, tok: &Token) {
        self.err_list.push(ParseError::new(
            Error::UnexpectedToken,
            format!("expected {}, but found {} instead", *tok, *self.next_token),
        ));
    }

    fn error_no_prefix(&mut self) {
        self.err_list.push(ParseError::new(
            Error::UnexpectedToken,
            format!("no prefix for {:?} found", *self.current_token,),
        ));
    }

    pub fn next_token(&mut self) {
        self.current_token = mem::replace(&mut self.next_token, Box::new(self.lexer.next_token()));
    }

    fn next_token_is(&self, tok: &Token) -> bool {
        *self.next_token == *tok
    }

    fn expect_next_token(&mut self, token: Token) -> bool {
        if self.next_token_is(&token) {
            self.next_token();
            true
        } else {
            self.error_next(&token);
            false
        }
    }

    fn token_order(token: &Token) -> Order {
        match token {
            Token::Equal | Token::NotEqual => Order::Equals,
            Token::LessThan
            | Token::LessThanAndEqual
            | Token::MoreThan
            | Token::MoreThanAndEqual => Order::LessGreater,
            Token::Plus | Token::Minus => Order::Sum,
            Token::Asterisk | Token::Slash => Order::Product,
            Token::LeftBracket => Order::Index,
            Token::LeftParanthesis => Order::Call,
            Token::Assign => Order::Assign,
            _ => Order::Lowest,
        }
    }

    fn peek_order(&self) -> Order {
        Self::token_order(&self.next_token)
    }

    fn current_order(&self) -> Order {
        Self::token_order(&self.current_token)
    }

    pub fn parse(&mut self) -> Program {
        let mut program: Program = vec![];

        while *self.current_token != Token::Eof {
            match self.parse_statement() {
                Some(stmt) => program.push(stmt),
                None => {}
            }
            self.next_token();
        }
        program
    }

    fn parse_identifier(&mut self) -> Option<String> {
        match *self.current_token {
            Token::Identifier(ref mut ident) => Some(ident.to_string()),
            _ => None,
        }
    }

    fn parse_statement(&mut self) -> Option<ParseItem::Statement> {
        if *self.next_token == Token::Let {
            self.next_token();
        }
        match (*self.current_token) {
            Token::Return => self.parse_return_statement(),
            Token::Let => self.parse_let_statement(),
            Token::Illegal => Some(ParseItem::Statement::None),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_expression_statement(&mut self) -> Option<ParseItem::Statement> {
        match self.parse_expression(Order::Lowest) {
            Some(expr) => Some(ParseItem::Statement::Expression(expr)),
            None => None,
        }
    }

    fn parse_expression(&mut self, order: Order) -> Option<ParseItem::Expression> {
        let mut left = match *self.current_token {
            Token::Identifier(_) => self.parse_identifier_expression(),
            Token::True | Token::False => self.parse_bool_expression(),
            Token::Int(_) => self.parse_int_expression(),
            Token::LeftBracket => self.parse_array_expression(),
            Token::Fn => self.parse_function_expression(),
            Token::Exclamation | Token::Minus | Token::Plus => self.parse_prefix_expression(),
            Token::Semicolon => {
                self.next_token();
                return None;
            }
            Token::If => self.parse_if_expression(),
            _ => {
                self.error_no_prefix();
                return None;
            }
        };
        while !self.next_token_is(&Token::Semicolon) && order < self.peek_order() {
            match *self.next_token {
                Token::Plus
                | Token::Minus
                | Token::Slash
                | Token::Asterisk
                | Token::Equal
                | Token::NotEqual
                | Token::LessThan
                | Token::LessThanAndEqual
                | Token::MoreThan
                | Token::MoreThanAndEqual
                | Token::Assign => {
                    self.next_token();
                    left = self.parse_infix_expression(left.unwrap());
                    // if *self.next_token == Token::Semicolon {
                    //     self.next_token();
                    // }
                }
                Token::Identifier(_) => {
                    self.next_token();
                    self.error_no_prefix();
                }
                Token::LeftBracket => {
                    self.next_token();
                    left = self.parse_index_expression(left.unwrap());
                }
                Token::LeftParanthesis => {
                    self.next_token();
                    left = self.parse_call_expression(left.unwrap());
                }
                _ => return left,
            }
        }

        if *self.next_token == Token::Eof
            && *self.current_token != Token::Eof
            && *self.current_token != Token::Semicolon
            && *self.current_token != Token::RightBrace
        {
            self.error_next(&Token::Semicolon);
            self.next_token();
            return None;
        }
        left
    }

    fn parse_prefix_expression(&mut self) -> Option<ParseItem::Expression> {
        let prefix = match *self.current_token {
            Token::Exclamation => ParseItem::Prefix::Not,
            Token::Plus => ParseItem::Prefix::Plus,
            Token::Minus => ParseItem::Prefix::Minus,
            _ => {
                self.error_no_prefix();
                return None;
            }
        };

        self.next_token();

        match self.parse_expression(Order::Prefix) {
            Some(expr) => Some(ParseItem::Expression::Prefix(prefix, Box::new(expr))),
            None => None,
        }
    }

    fn parse_infix_expression(
        &mut self,
        left: ParseItem::Expression,
    ) -> Option<ParseItem::Expression> {
        let infix = match *self.current_token {
            Token::Plus => ParseItem::Infix::Plus,
            Token::Minus => ParseItem::Infix::Minus,
            Token::Asterisk => ParseItem::Infix::Multiply,
            Token::Slash => ParseItem::Infix::Divide,
            Token::Equal => ParseItem::Infix::Equal,
            Token::NotEqual => ParseItem::Infix::NotEqual,
            Token::LessThanAndEqual => ParseItem::Infix::LessThanAndEqual,
            Token::LessThan => ParseItem::Infix::LessThan,
            Token::MoreThanAndEqual => ParseItem::Infix::MoreThanAndEqual,
            Token::MoreThan => ParseItem::Infix::MoreThan,
            Token::Assign => ParseItem::Infix::Assign,
            _ => return None,
        };

        let order = self.current_order();

        self.next_token();
        match self.parse_expression(order) {
            Some(expression) => {
                //self.next_token();
                Some(ParseItem::Expression::Infix(
                    infix,
                    Box::new(left),
                    Box::new(expression),
                ))
            }
            None => None,
        }
    }

    fn parse_index_expression(
        &mut self,
        left: ParseItem::Expression,
    ) -> Option<ParseItem::Expression> {
        self.next_token();
        let index = match self.parse_expression(Order::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        if !self.expect_next_token(Token::RightBracket) {
            return None;
        }

        Some(ParseItem::Expression::Index(
            Box::new(left),
            Box::new(index),
        ))
    }

    fn parse_identifier_expression(&mut self) -> Option<ParseItem::Expression> {
        match self.parse_identifier() {
            Some(ident) => Some(ParseItem::Expression::Identifier(ident)),
            _ => return None,
        }
    }

    fn parse_integer_literal(&mut self) -> Option<ParseItem::Expression> {
        match *self.current_token {
            Token::Int(ref mut int) => Some(ParseItem::Expression::Integer(int.clone())),
            _ => return None,
        }
    }

    fn parse_bool_expression(&mut self) -> Option<ParseItem::Expression> {
        match *self.current_token {
            Token::True => Some(ParseItem::Expression::Bool(true)),
            Token::False => Some(ParseItem::Expression::Bool(false)),
            _ => None,
        }
    }

    fn parse_int_expression(&mut self) -> Option<ParseItem::Expression> {
        match *self.current_token {
            Token::Int(ref mut int) => Some(ParseItem::Expression::Integer(*int)),
            _ => None,
        }
    }

    fn parse_expression_list(&mut self, end: Token) -> Option<Vec<ParseItem::Expression>> {
        let mut vec = vec![];

        if self.next_token_is(&end) {
            self.next_token();
            return Some(vec);
        }

        self.next_token();

        match self.parse_expression(Order::Lowest) {
            Some(expr) => vec.push(expr),
            _ => return None,
        }
        while self.next_token_is(&Token::Comma) {
            self.next_token();
            self.next_token();
            match self.parse_expression(Order::Lowest) {
                Some(expr) => vec.push(expr),
                _ => return None,
            }
        }
        if *self.next_token != end {
            return None;
        }
        Some(vec)
    }

    fn parse_array_expression(&mut self) -> Option<ParseItem::Expression> {
        if !self.next_token_is(&Token::LeftBrace) {
            self.error_next(&Token::LeftBrace);
        }
        let mut arr: Vec<Vec<ParseItem::Expression>> = vec![];
        while *self.next_token != Token::RightBracket {
            self.next_token();
            let vec = self.parse_expression_list(Token::RightBrace);
            arr.push(vec.unwrap());
            self.next_token();
            if *self.current_token != Token::Comma
                && *self.current_token != Token::RightBrace
                && *self.current_token != Token::RightBracket
            {
                // self.error_no_prefix();
                return None;
            }
        }
        self.next_token();
        Some(ParseItem::Expression::Array(arr))
    }

    fn parse_function_parameters(&mut self) -> Option<Vec<String>> {
        let mut params = vec![];
        if *self.next_token == Token::RightParanthesis {
            return Some(params);
        }
        self.next_token();
        match self.parse_identifier() {
            Some(ident) => params.push(ident),
            _ => return None,
        };
        while self.next_token_is(&Token::Comma) {
            self.next_token();
            self.next_token();
            match self.parse_identifier() {
                Some(ident) => params.push(ident),
                _ => return None,
            };
        }
        if !self.expect_next_token(Token::RightParanthesis) {
            self.error_next(&Token::RightParanthesis);
            return None;
        }

        Some(params)
    }

    fn parse_function_expression(&mut self) -> Option<ParseItem::Expression> {
        self.next_token();
        if *self.current_token == Token::LeftParanthesis {
            self.error_next(&Token::LeftParanthesis);
            return None;
        }

        let ident = match self.parse_identifier() {
            Some(ident) => ident,
            _ => return None,
        };
        self.next_token();

        let params = match self.parse_function_parameters() {
            Some(params) => params,
            _ => return None,
        };

        if !self.expect_next_token(Token::LeftBrace) {
            self.error_next(&Token::LeftBrace);
            return None;
        }

        let body = self.parse_block_statements();

        //self.next_token();
        Some(ParseItem::Expression::Function(ident, params, body))
    }

    fn parse_block_statements(&mut self) -> Vec<ParseItem::Statement> {
        let mut statements: Vec<ParseItem::Statement> = vec![];

        self.next_token();

        while *self.current_token != Token::RightBrace && *self.current_token != Token::Eof {
            match self.parse_statement() {
                Some(statement) => {
                    if *self.next_token != Token::Semicolon {
                        self.error_next(&Token::Semicolon);
                        *self.current_token = Token::Eof;
                        *self.next_token = Token::Eof;
                        self.next_token();
                        ()
                    }

                    statements.push(statement);
                    self.next_token();
                }
                None => (),
            }
            self.next_token();
        }

        statements
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

        if *self.next_token == Token::Semicolon {
            return Some(ParseItem::Statement::Let(
                ident,
                ParseItem::Expression::Integer(0),
            ));
        }

        if !self.expect_next_token(Token::Assign) {
            self.next_token();
            return None;
        }

        self.next_token();
        let eval = match self.parse_expression(Order::Lowest) {
            Some(expr) => expr,
            _ => return None,
        };
        self.next_token();

        Some(ParseItem::Statement::Let(ident, eval))
    }

    fn parse_return_statement(&mut self) -> Option<ParseItem::Statement> {
        self.next_token();

        let expression = match self.parse_expression(Order::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        // if self.next_token_is(&Token::Semicolon) {
        //     self.next_token();
        // }

        Some(ParseItem::Statement::Return(expression))
    }

    fn parse_if_expression(&mut self) -> Option<ParseItem::Expression> {
        if !self.expect_next_token(Token::LeftParanthesis) {
            return None;
        }

        self.next_token();

        let predicate = match self.parse_expression(Order::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        if !self.expect_next_token(Token::RightParanthesis)
            || !self.expect_next_token(Token::LeftBrace)
        {
            return None;
        }

        let body = self.parse_block_statements();
        let mut else_body = None;

        if self.next_token_is(&Token::Else) {
            self.next_token();

            if !self.expect_next_token(Token::LeftBrace) {
                return None;
            }

            else_body = Some(self.parse_block_statements());
        }

        Some(ParseItem::Expression::If(
            Box::new(predicate),
            body,
            else_body,
        ))
    }

    pub fn parse_call_expression(
        &mut self,
        expr: ParseItem::Expression,
    ) -> Option<ParseItem::Expression> {
        let args = match self.parse_expression_list(Token::RightParanthesis) {
            Some(args) => args,
            None => return None,
        };

        Some(ParseItem::Expression::Call {
            func: Box::new(expr),
            args,
        })
    }
}
