use std::{env, fs};

mod ast;
mod lexer;
mod parser;
mod token;
mod token_def;
mod visitor;

use inkwell::context::Context;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::visitor::Visitor;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("lang filename");
        return;
    }

    let input = fs::read_to_string(&args[1]).unwrap();

    let lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer).unwrap();

    let ast = parser.parse().unwrap();
    println!("{:?}", ast);
    let visitor = Visitor::default();

    let _context = Context::create();

    let result = visitor.visit(&ast);
    println!("{}", result);
}
