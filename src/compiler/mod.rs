use crate::{
    ast::{ExpressionNode, Node, StatementNode},
    code::{make::make, Instructions, Opcode},
    object::Object,
};

pub struct Compiler {
    instructions: Instructions,
    constants: Vec<Object>,
}

pub struct Bytecode {
    instructions: Instructions,
    constants: Vec<Object>,
}

type R = Result<(), &'static str>;

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            instructions: Instructions(vec![]),
            constants: vec![],
        }
    }

    pub fn compile(&mut self, node: Node) -> R {
        match node {
            Node::Statement(_) => todo!(),
            Node::Expression(_) => todo!(),
            Node::Program(node) => self.compile_statements(&node.statements),
        }
    }

    fn compile_statements(&mut self, statements: &[StatementNode]) -> R {
        for statement in statements {
            self.compile_statement(statement)?;
        }

        Ok(())
    }

    fn compile_statement(&mut self, statement: &StatementNode) -> R {
        match statement {
            StatementNode::LetStatement(_) => todo!(),
            StatementNode::ReturnStatement(_) => todo!(),
            StatementNode::BlockStatement(_) => todo!(),
            StatementNode::ExpressionStatement(node) => self.compile_expression(&node.expression),
        }
    }

    fn compile_expression(&mut self, expression: &ExpressionNode) -> R {
        match expression {
            ExpressionNode::Identifier(_) => todo!(),
            ExpressionNode::IntegerLiteral(i) => {
                let integer = Object::Integer(i.value);
                let pos = self.add_constant(integer);

                self.emit(Opcode::OpConstant, vec![pos]);

                Ok(())
            }
            ExpressionNode::BooleanLiteral(_) => todo!(),
            ExpressionNode::StringLiteral(_) => todo!(),
            ExpressionNode::ArrayLiteral(_) => todo!(),
            ExpressionNode::PrefixExpression(_) => todo!(),
            ExpressionNode::InfixExpression(node) => {
                self.compile_expression(&node.left)?;
                self.compile_expression(&node.right)
            }
            ExpressionNode::IfExpression(_) => todo!(),
            ExpressionNode::FunctionExpression(_) => todo!(),
            ExpressionNode::CallExpression(_) => todo!(),
            ExpressionNode::IndexExpresssion(_) => todo!(),
            ExpressionNode::HashLiteral(_) => todo!(),
        }
    }

    fn emit(&mut self, op: Opcode, operands: Vec<usize>) -> usize {
        let instruction = make(op, &operands);
        self.add_instruction(instruction)
    }

    fn add_instruction(&mut self, mut instruction: Vec<u8>) -> usize {
        let pos = self.instructions.0.len();
        self.instructions.0.append(&mut instruction);
        pos
    }

    fn add_constant(&mut self, object: Object) -> usize {
        self.constants.push(object);

        self.constants.len() - 1
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
                make(Opcode::OpConstant, &[0]),
                make(Opcode::OpConstant, &[1]),
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

        let expected_instructions = expected_instructions
            .into_iter()
            .flatten()
            .collect::<Vec<u8>>()
            .into();

        assert_eq!(bytecode.instructions, expected_instructions);

        assert_eq!(expected_constants.len(), bytecode.constants.len());

        for (expected, result) in expected_constants.iter().zip(bytecode.constants) {
            test_object(&result, expected)
        }
    }
}