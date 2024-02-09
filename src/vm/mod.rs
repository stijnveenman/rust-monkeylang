use crate::{compiler::Bytecode, object::Object};

pub struct Vm {
    bytecode: Bytecode,
}

impl Vm {
    fn new(bytecode: Bytecode) -> Vm {
        Vm { bytecode }
    }

    fn stack_stop(&self) -> Object {
        todo!()
    }

    fn run(&self) {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use std::any::Any;

    use rstest::rstest;

    use crate::{compiler::Compiler, object::test::test_object, parser::Parser, vm::Vm};

    #[rstest]
    #[case("1", 1)]
    #[case("2", 2)]
    #[case("1 + 2", 2)] //FIXME
    fn test_integer_arithmetic(#[case] input: &str, #[case] expected: i32) {
        test_vm(input, expected)
    }

    fn test_vm<T: Any>(input: &str, expected: T) {
        let mut parser = Parser::new(input.into());
        let (program, errors) = parser.parse_program();

        assert_eq!(errors, Vec::<String>::new());

        let mut compiler = Compiler::new();
        compiler
            .compile((&program).into())
            .expect("Failed to compile program");

        let vm = Vm::new(compiler.bytecode());
        vm.run();

        let element = vm.stack_stop();

        test_object(&element, &expected)
    }
}
