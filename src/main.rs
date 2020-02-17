mod lexer_mod;
use std::fs::File;
use std::io::{BufRead, BufReader};
use lexer_mod::token::Token;
use lexer_mod::lexer::Lexer;

fn main() {
    let file = File::open("progr.txt").unwrap();

    let reader = BufReader::new(file);

    for val in reader.lines() {
        let mut line = val.unwrap();
        let mut lexer = Lexer::new(&mut line);
        loop {
            let tok = lexer.next_token();
            println!("{:?}", tok);
            if tok == Token::Eof {
            break;
            }
        }
    }
}
