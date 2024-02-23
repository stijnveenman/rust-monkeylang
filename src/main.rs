pub mod ast;
pub mod builtin;
pub mod code;
pub mod compiler;
pub mod evaluator;
pub mod object;
pub mod parser;
pub mod repl;
pub mod tokens;
pub mod vm;

use std::{env, fs, process::exit};

use compiler::Compiler;
use repl::repl_run;
use vm::Vm;

use crate::{
    evaluator::{environment::Environment, eval},
    parser::Parser,
};

fn run(file: &str) {
    let content = fs::read_to_string(file).unwrap();

    let mut parser = Parser::new(content);

    let (program, errors) = parser.parse_program();

    if !errors.is_empty() {
        println!("ERRORS: {:?}", errors);
        exit(1);
    }

    let result = eval(&Environment::new(), (&program).into());
    println!("{}", result);
}

fn compiled_run(file: &str) {
    let mut compiler = Compiler::new();
    let mut vm = Vm::new();

    let content = fs::read_to_string(file).unwrap();

    match repl_run(&mut compiler, &mut vm, content) {
        Ok(result) => println!("{}", result),
        Err(err) => {
            println!("ERR: {}", err);
            exit(1);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        if args.get(1).unwrap() == "-c" {
            compiled_run(args.get(2).unwrap())
        } else {
            run(args.get(1).unwrap())
        }
    } else {
        repl::start();
    }
}
