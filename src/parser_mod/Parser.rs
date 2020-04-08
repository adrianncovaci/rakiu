use super::ParseItem;
use crate::lexer_mod::lexer;
use crate::lexer_mod::lexer::Lexer;
use crate::lexer_mod::token::Token;

use std::mem;
use std::fmt;

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
        Parser {
            lexer: lexer_,
            current_token: Box::new(Token::Illegal),
            next_token: Box::new(Token::Illegal),
            err_list: Vec::new(),
        }
    }

    pub fn get_errors(&mut self) -> ParseErrors {
        self.err_list.clone()
    }

    fn error_next(&mut self, tok: Token) {
        self.err_list.push(ParseError::new(
            Error::UnexpectedToken,
            format!(
                "expected {:?}, but found {:?} instead",
                tok, *self.current_token
            ),
        ));
    }

    fn error_no_prefix(&mut self) {
        self.err_list.push(ParseError::new(     Error::UnexpectedToken,
            format!(
                "no prefix for {:?} found",
                *self.current_token,
            )
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
            self.error_next(token);
            false
        }
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
            Token::Identifier(ref mut ident) => Some(ident.to_string()),
            _ => None
        }
    }

    pub fn parse_statement(&mut self) -> Option<ParseItem::Statement> {
        match(*self.current_token) {
            Token::Let => self.parse_let_statement(),
            _ => None
        }
    }

    fn peek_order(&self) -> Order {
        token_order(&self.next_token)
    }

    fn current_order(&self) -> Order {
        token_order(&self.current_token)
    }

    fn parse_expression(&mut self, order: Order) -> Option<ParseItem::Expression> {
        let prefix_expr = self.parse_prefix_expression();
        println!("{:?}", *self.current_token);
        let mut left = match *self.current_token {
            Token::Identifier(_) => self.parse_identifier_expression(),
            Token::And | Token::False => self.parse_bool_expression(),
            Token::Int(_) => self.parse_int_expression(),
            Token::LeftBrace => self.parse_array_expression(),
            Token::Fn => self.parse_function_expression(),
            Token::Exclamation | Token::Minus | Token::Plus => self.parse_prefix_expression(),
            //Token::If => self.parse_if_expression(), //
            _ => {
                self.error_no_prefix();
                return None;
            }
         };

        println!("{:?}", *self.next_token==Token::Assign);
        while !self.expect_next_token(Token::Semicolon) && order < self.peek_order()
            && token_order(&self.next_token) != Order::Lowest {
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
                    left = self.parse_call_expression(left.unwrap());
                }
                _ => return left,
            }
        }
            left
    }

    fn parse_prefix_expression(&mut self) -> Option<ParseItem::Expression> {
        let prefix = match *self.current_token {
            Token::Exclamation => ParseItem::Prefix::Not,
            Token::Plus => ParseItem::Prefix::Plus,
            Token::Minus => ParseItem::Prefix::Minus,
            _ => return None,
        };

        self.next_token();

        match self.parse_expression(Order::Prefix) {
            Some(expr) => Some(ParseItem::Expression::Prefix(prefix, Box::new(expr))),
            None => None,
        }
    }

    fn parse_infix_expression(&mut self, left: ParseItem::Expression) -> Option<ParseItem::Expression> {
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
            _ => return None,
        };

        let order = self.current_order();

        self.next_token();

        match self.parse_expression(order) {
            Some(expression) => Some(ParseItem::Expression::Infix(infix, Box::new(left), Box::new(expression))),
            None => {
                None
            }
        }
    }

    fn parse_index_expression(&mut self, left: ParseItem::Expression) -> Option<ParseItem::Expression> {
        self.next_token();
        let index = match self.parse_expression(Order::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        if !self.expect_next_token(Token::RightBrace) {
            return None;
        }

        Some(ParseItem::Expression::Index(Box::new(left), Box::new(index)))
    }

    fn parse_identifier_expression(&mut self) -> Option<ParseItem::Expression> {
        match self.parse_identifier() {
            Some(ident) => Some(ParseItem::Expression::Identifier(ident)),
            _ => return None,
        }
    }

    fn parse_integer_literal(&mut self, tok: Token) -> Option<ParseItem::Expression> {
        match *self.current_token {
            Token::Int(ref mut int) => Some(ParseItem::Expression::Integer(int.clone())),
            _ => return None,
        }
    }

    fn parse_bool_expression(&mut self) -> Option<ParseItem::Expression> {
        match *self.current_token {
            Token::True => Some(ParseItem::Expression::Bool(true)),
            Token::False => Some(ParseItem::Expression::Bool(false)),
            _ =>  None,
        }
    }

    fn parse_int_expression(&mut self) -> Option<ParseItem::Expression> {
        match *self.current_token {
            Token::Int(ref mut int) => Some(ParseItem::Expression::Integer(*int)),
            _ => None,
        }
    }

    fn parse_expression_list(&mut self, end:Token) -> Option<Vec<ParseItem::Expression>> {
        let mut vec = vec![];

        if self.next_token_is(&end) {
            return Some(vec);
        }

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

        if !self.expect_next_token(end) {
            self.error_next(Token::RightParanthesis);
            return None;
        }
        Some(vec)

    }

    fn parse_array_expression(&mut self) -> Option<ParseItem::Expression> {
        match self.parse_expression_list(Token::RightBrace) {
            Some(vec) => Some(ParseItem::Expression::Array(vec)),
            _ => None,
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
            _ => return None,
        };

        while self.expect_next_token(Token::Comma) {
            self.next_token();
            self.next_token();

            match self.parse_identifier() {
                Some(ident) => params.push(ident),
                _ => return None,
            };
        }

        if !self.expect_next_token(Token::RightParanthesis) {
            return None;
        }

        Some(params)
    }

    fn parse_function_expression(&mut self) -> Option<ParseItem::Expression> {
        if !self.expect_next_token(Token::LeftParanthesis) {
            self.error_next(Token::LeftParanthesis);
            return None;
        }

        let params = match self.parse_function_parameters() {
            Some(params) => params,
            _ => return None,
        };

        if !self.expect_next_token(Token::LeftBrace) {
            self.error_next(Token::LeftParanthesis);
            return None;
        }

        let body = self.parse_block_statements();

        Some(ParseItem::Expression::Function(params, Box::new(body)))
    }

    fn parse_block_statements(&mut self) -> ParseItem::Statement {
        let mut statements: Vec<ParseItem::Statement> = vec![];

        self.next_token();

        while !self.expect_next_token(Token::RightBrace) {
            match self.parse_statement() {
                Some(statement) => statements.push(statement),
                None => (),
            }
            self.next_token();
        }

        ParseItem::Statement::BlockStatement(statements)
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

        self.next_token();
        println!("{:?}", *self.current_token);
        let eval = match self.parse_expression(Order::Lowest) {
            Some(expr) => expr,
            _ =>  return None,
        };

        while (*self.current_token) != Token::Semicolon {
            self.next_token();
        }
        Some(ParseItem::Statement::Let(ident, eval))
    }

    pub fn parse_call_expression(&mut self, expr: ParseItem::Expression) -> Option<ParseItem::Expression> {
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
