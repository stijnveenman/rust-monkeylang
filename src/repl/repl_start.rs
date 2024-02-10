use std::io::{self, stdout, BufRead, Write};

use crate::{compiler::Compiler, parser::Parser, vm::Vm};

const PROMPT: &str = ">>";

fn run(line: String) -> Result<String, String> {
    let mut parser = Parser::new(line);

    let (program, errors) = parser.parse_program();

    if !errors.is_empty() {
        Err(errors.join("\n").to_string())?;
    }

    let mut compiler = Compiler::new();
    compiler.compile((&program).into())?;

    let mut vm = Vm::new(compiler.bytecode());
    vm.run()?;

    let result = vm.last_popped();
    Ok(format!("{}", result))
}

pub fn start() {
    let stdin = io::stdin();

    print!("{}", PROMPT);
    stdout().flush().expect("failed to flush stdout");

    for line in stdin.lock().lines() {
        let line = line.expect("failed to read line from stdin");

        match run(line) {
            Ok(result) => println!("{result}"),
            Err(e) => println!("{e}"),
        }

        print!("{}", PROMPT);
        stdout().flush().expect("failed to flush stdout");
    }
}
