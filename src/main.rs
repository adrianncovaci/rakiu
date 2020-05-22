mod evaluation_mod;
mod lexer_mod;
mod parser_mod;
use crate::evaluation_mod::codegen::generate_code;
use crate::evaluation_mod::env::Env;
use crate::evaluation_mod::evaluate::eval_statements;
use lexer_mod::lexer::Lexer;
use parser_mod::Parser::Parser;
use std::io;

fn main() {
    let mut input = String::new();
    while input.trim() != "exit" {
        io::stdin()
            .read_line(&mut input)
            .expect("Could not read from stdin");
        let lexer = Lexer::new(&mut input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse();
        let errs = parser.get_errors();
        println!("{}", &program[program.len() - 1]);

        if errs.len() > 0 {
            for el in &errs {
                println!("{}", el);
            }
        }
        let mut env = Env::new();
        // unsafe {
        //     generate_code(program);
        // }
        eval_statements(program, &mut env);
    }
}
