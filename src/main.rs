mod lexer_mod;
mod parser_mod;
use std::fs::File;
use std::io::{BufRead, BufReader};
use lexer_mod::token::Token;
use lexer_mod::lexer::Lexer;
use parser_mod::Parser::Parser;
use parser_mod::ParseItem::Program;

fn main() {
    let file = File::open("progr.txt").unwrap();

    let reader = BufReader::new(file);

    for val in reader.lines() {
        let mut line = val.unwrap();
        let mut lexer = Lexer::new(&mut line);
        let mut parser = Parser::new(lexer);
        let program = parser.parse();
        let errs = parser.get_errors();
        for ls in &program {
                println!("{}", ls);
        }

        if errs.len() == 0 {
            println!("Parsing successful!");
        } else {
            for el in &errs {
                println!("{}", el);
            }
        }
    }
}
