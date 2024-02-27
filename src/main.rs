use std::{env, fs};

mod ast;
mod lexer;
mod parser;
mod token;
mod token_def;
mod visitor;

use inkwell::context::Context;
use inkwell::execution_engine::JitFunction;
use inkwell::targets::{
    CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine,
};
use inkwell::OptimizationLevel;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::visitor::Visitor;

pub type SumFunc = unsafe extern "C" fn() -> u64;

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

    let context = Context::create();
    let module = context.create_module("test_module");
    let builder = context.create_builder();
    let exe_engine = module
        .create_jit_execution_engine(OptimizationLevel::None)
        .unwrap();

    let mut visitor = Visitor::new(&context, module, builder);

    visitor.visit(&ast);

    unsafe {
        let fun: JitFunction<'_, SumFunc> = exe_engine.get_function("sum").unwrap();

        Target::initialize_native(&InitializationConfig::default())
            .expect("Failed to initialize native target");

        let triple = TargetMachine::get_default_triple();
        let cpu = TargetMachine::get_host_cpu_name().to_string();
        let features = TargetMachine::get_host_cpu_features().to_string();

        let target = Target::from_triple(&triple).unwrap();
        let machine = target
            .create_target_machine(
                &triple,
                &cpu,
                &features,
                OptimizationLevel::Aggressive,
                RelocMode::Default,
                CodeModel::Default,
            )
            .unwrap();

        // create a module and do JIT stuff

        machine
            .write_to_file(&visitor.module, FileType::Object, "out.o".as_ref())
            .unwrap();

        let res = fun.call();
        println!("{}", res);
    }
}
