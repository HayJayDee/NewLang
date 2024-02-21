use std::{env, fs};

use lexer::Lexer;

mod lexer;
mod token;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("lang filename");
        return;
    }
    
    let input = fs::read_to_string(&args[1]).unwrap();

    let lexer = Lexer::new(input.to_string());
    let tokens = lexer.lex();
    println!("{:?}", tokens);
}
