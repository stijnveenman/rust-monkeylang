use crate::{
    ast::{ExpressionNode, Node, StatementNode},
    code::{make::make, Instructions, Opcode},
    object::Object,
    tokens::token::Token,
};

pub struct Compiler {
    instructions: Instructions,
    constants: Vec<Object>,

    previous_instruction: (Opcode, usize),
    last_instruction: (Opcode, usize),
}

pub struct Bytecode {
    pub instructions: Instructions,
    pub constants: Vec<Object>,
}

type R = Result<(), String>;

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            instructions: Instructions(vec![]),
            constants: vec![],

            previous_instruction: (Opcode::OpPop, 0),
            last_instruction: (Opcode::OpPop, 0),
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
            StatementNode::BlockStatement(node) => self.compile_statements(&node.statements),
            StatementNode::ExpressionStatement(node) => {
                self.compile_expression(&node.expression)?;
                self.emit(Opcode::OpPop, vec![]);

                Ok(())
            }
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
            ExpressionNode::BooleanLiteral(node) => {
                if node.value {
                    self.emit(Opcode::OpTrue, vec![]);
                } else {
                    self.emit(Opcode::OpFalse, vec![]);
                }
                Ok(())
            }
            ExpressionNode::StringLiteral(_) => todo!(),
            ExpressionNode::ArrayLiteral(_) => todo!(),
            ExpressionNode::PrefixExpression(node) => {
                self.compile_expression(&node.right)?;

                match &node.operator {
                    Token::BANG => self.emit(Opcode::OpBang, vec![]),
                    Token::MINUS => self.emit(Opcode::OpMinus, vec![]),
                    e => Err(format!("unknown prefix operator {e:?}"))?,
                };

                Ok(())
            }
            ExpressionNode::InfixExpression(node) => {
                if node.operator.is(&Token::LT) {
                    // invert left and right on less then
                    self.compile_expression(&node.right)?;
                    self.compile_expression(&node.left)?;
                } else {
                    self.compile_expression(&node.left)?;
                    self.compile_expression(&node.right)?;
                }

                match &node.operator {
                    Token::PLUS => self.emit(Opcode::OpAdd, vec![]),
                    Token::MINUS => self.emit(Opcode::OpSub, vec![]),
                    Token::ASTERISK => self.emit(Opcode::OpMul, vec![]),
                    Token::SLASH => self.emit(Opcode::OpDiv, vec![]),

                    Token::GT | Token::LT => self.emit(Opcode::OpGreaterThan, vec![]),
                    Token::EQ => self.emit(Opcode::OpEqual, vec![]),
                    Token::NOT_EQ => self.emit(Opcode::OpNotEqual, vec![]),
                    e => Err(format!("unknown infix operator {e:?}"))?,
                };

                Ok(())
            }
            ExpressionNode::IfExpression(node) => {
                self.compile_expression(&node.condition)?;

                // add placeholder OpJumpNotTruthy
                let jump_not_truthy_pos = self.emit(Opcode::OpJumpNotTruthy, vec![9999]);

                self.compile_statements(&node.concequence.statements)?;

                if self.last_instruction.0.is_pop() {
                    self.remove_last();
                }

                let concequence_pos = self.instructions.0.len();
                self.change_operand(jump_not_truthy_pos, concequence_pos);

                Ok(())
            }
            ExpressionNode::FunctionExpression(_) => todo!(),
            ExpressionNode::CallExpression(_) => todo!(),
            ExpressionNode::IndexExpresssion(_) => todo!(),
            ExpressionNode::HashLiteral(_) => todo!(),
        }
    }

    fn emit(&mut self, op: Opcode, operands: Vec<usize>) -> usize {
        let instruction = make(op, &operands);
        let pos = self.add_instruction(instruction);

        self.set_last_instruction(op, pos);

        pos
    }

    fn replace_instruction(&mut self, pos: usize, instruction: Vec<u8>) {
        for (i, b) in instruction.into_iter().enumerate() {
            self.instructions.0[pos + i] = b;
        }
    }

    fn change_operand(&mut self, op_pos: usize, operand: usize) {
        let op: Opcode = self.instructions.0[op_pos].into();

        let instruction = make(op, &[operand]);

        self.replace_instruction(op_pos, instruction);
    }

    fn remove_last(&mut self) {
        self.instructions.0 = self.instructions.0[..self.last_instruction.1].to_vec();

        self.last_instruction = self.previous_instruction;
    }

    fn set_last_instruction(&mut self, op: Opcode, pos: usize) {
        let previous = self.last_instruction;
        self.last_instruction = (op, pos);

        self.previous_instruction = previous;
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

    use rstest::rstest;

    use crate::{
        code::{make::make, Opcode},
        compiler::Compiler,
        object::test::test_object,
        parser::Parser,
    };

    #[rstest]
    #[case("1 + 2",vec![1,2],vec![
        make(Opcode::OpConstant,&[0]),
        make(Opcode::OpConstant,&[1]),
        make(Opcode::OpAdd,&[]),
        make(Opcode::OpPop,&[]),
    ])]
    #[case("1 - 2",vec![1,2],vec![
        make(Opcode::OpConstant,&[0]),
        make(Opcode::OpConstant,&[1]),
        make(Opcode::OpSub,&[]),
        make(Opcode::OpPop,&[]),
    ])]
    #[case("1 * 2",vec![1,2],vec![
        make(Opcode::OpConstant,&[0]),
        make(Opcode::OpConstant,&[1]),
        make(Opcode::OpMul,&[]),
        make(Opcode::OpPop,&[]),
    ])]
    #[case("2 / 1",vec![2,1],vec![
        make(Opcode::OpConstant,&[0]),
        make(Opcode::OpConstant,&[1]),
        make(Opcode::OpDiv,&[]),
        make(Opcode::OpPop,&[]),
    ])]
    #[case("1; 2",vec![1,2],vec![
        make(Opcode::OpConstant,&[0]),
        make(Opcode::OpPop,&[]),
        make(Opcode::OpConstant,&[1]),
        make(Opcode::OpPop,&[]),
    ])]
    #[case("-1",vec![1],vec![
        make(Opcode::OpConstant,&[0]),
        make(Opcode::OpMinus,&[]),
        make(Opcode::OpPop,&[]),
    ])]
    fn test_integer_arithmetic(
        #[case] input: &str,
        #[case] constants: Vec<i64>,
        #[case] instructions: Vec<Vec<u8>>,
    ) {
        test_compiler(input, constants, instructions)
    }

    #[rstest]
    #[case("true", vec![], vec![make(Opcode::OpTrue, &[]), make(Opcode::OpPop, &[])])]
    #[case("false", vec![], vec![make(Opcode::OpFalse, &[]), make(Opcode::OpPop, &[])])]
    #[case("1 > 2", vec![1, 2], vec![make(Opcode::OpConstant, &[0]), make(Opcode::OpConstant, &[1]), make(Opcode::OpGreaterThan, &[]), make(Opcode::OpPop, &[])])]
    #[case("1 < 2", vec![2, 1], vec![make(Opcode::OpConstant, &[0]), make(Opcode::OpConstant, &[1]), make(Opcode::OpGreaterThan, &[]), make(Opcode::OpPop, &[])])]
    #[case("1 == 2", vec![1, 2], vec![make(Opcode::OpConstant, &[0]), make(Opcode::OpConstant, &[1]), make(Opcode::OpEqual, &[]), make(Opcode::OpPop, &[])])]
    #[case("1 != 2", vec![1, 2], vec![make(Opcode::OpConstant, &[0]), make(Opcode::OpConstant, &[1]), make(Opcode::OpNotEqual, &[]), make(Opcode::OpPop, &[])])]
    #[case("true == true", vec![], vec![make(Opcode::OpTrue, &[]), make(Opcode::OpTrue, &[]), make(Opcode::OpEqual, &[]), make(Opcode::OpPop, &[])])]
    #[case("true != false", vec![], vec![make(Opcode::OpTrue, &[]), make(Opcode::OpFalse, &[]), make(Opcode::OpNotEqual, &[]), make(Opcode::OpPop, &[])])]
    #[case("!true", vec![], vec![make(Opcode::OpTrue, &[]), make(Opcode::OpBang, &[]), make(Opcode::OpPop, &[])])]
    fn test_boolean_expressions(
        #[case] input: &str,
        #[case] constants: Vec<i64>,
        #[case] instructions: Vec<Vec<u8>>,
    ) {
        test_compiler(input, constants, instructions)
    }

    #[rstest]
    #[case("if (true) {10}; 3333;", vec![10, 3333], vec![
        make(Opcode::OpTrue, &[]),
        make(Opcode::OpJumpNotTruthy, &[7]),
        make(Opcode::OpConstant, &[0]),
        make(Opcode::OpPop, &[]),
        make(Opcode::OpConstant, &[1]),
        make(Opcode::OpPop, &[]),
    ])]
    fn test_conditionals(
        #[case] input: &str,
        #[case] constants: Vec<i64>,
        #[case] instructions: Vec<Vec<u8>>,
    ) {
        test_compiler(input, constants, instructions)
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
