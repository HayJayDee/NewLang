use std::{env, fs};

mod lexer;
mod token;
mod token_def;

use lexer::Lexer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("lang filename");
        return;
    }

    let input = fs::read_to_string(&args[1]).unwrap();

    let lexer = Lexer::new(input.to_string());
    match lexer.lex() {
        Ok(tokens) => {
            println!("{:?}", tokens);
        },
        Err(err) => {
            println!("{}", err)
        }
    }
}
