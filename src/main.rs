use std::{env, fs, process::exit};

use rust_monkeylang::{
    evaluator::{environment::Environment, eval},
    parser::Parser,
    repl,
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

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        run(args.get(1).unwrap())
    } else {
        repl::start();
    }
}
