use crate::{ast::Node, object::Object};

pub struct Compiler {
    instructions: Vec<u8>,
    constants: Vec<Object>,
}

pub struct Bytecode {
    instructions: Vec<u8>,
    constants: Vec<Object>,
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            instructions: vec![],
            constants: vec![],
        }
    }

    pub fn compile(&mut self, node: Node) -> Result<(), &'static str> {
        Ok(())
    }

    pub fn bytecode(&self) -> Bytecode {
        Bytecode {
            instructions: self.instructions.clone(),
            constants: self.constants.clone(),
        }
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
pub mod test {
    use std::any::Any;

    use crate::{
        code::{make::make, Opcode},
        compiler::Compiler,
        object::test::test_object,
        parser::Parser,
    };

    #[test]
    fn test_integer_arithmetic() {
        test_compiler(
            "1 + 2",
            vec![1, 2],
            vec![
                make(Opcode::OpConstant, vec![0]),
                make(Opcode::OpConstant, vec![1]),
            ],
        )
    }

    pub fn test_compiler<T: Any>(
        input: &str,
        expected_constants: Vec<T>,
        expected_instructions: Vec<Vec<u8>>,
    ) {
        let mut parser = Parser::new(input.into());
        let (program, errors) = parser.parse_program();

        assert_eq!(errors, Vec::<String>::new());

        let mut compiler = Compiler::new();
        compiler
            .compile((&program).into())
            .expect("Failed to compile program");

        let bytecode = compiler.bytecode();

        assert_eq!(
            bytecode.instructions,
            expected_instructions
                .into_iter()
                .flatten()
                .collect::<Vec<u8>>()
        );

        for (expected, result) in expected_constants.iter().zip(bytecode.constants) {
            test_object(&result, expected)
        }
    }
}
