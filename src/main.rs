use std::{env, fs};

mod lexer;
mod token;
mod token_def;
mod parser;
mod ast;

use crate::parser::Parser;
use crate::lexer::Lexer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("lang filename");
        return;
    }

    let input = fs::read_to_string(&args[1]).unwrap();

    let mut lexer = Lexer::new(input.to_string());
    
    while let Some(token) = lexer.next() {
        println!("{:?}", token);
    }

    /*
    match lexer.lex() {
        Ok(tokens) => {
            println!("{:?}", tokens);
            let ast = Parser::parse(tokens);
            println!("{:?}", ast);
        }
        Err(err) => {
            println!("{}", err)
        }
    }
    */
}
