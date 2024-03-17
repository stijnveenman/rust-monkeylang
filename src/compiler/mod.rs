mod symbol_table;
mod symbol_stack;

use crate::{
    ast::{ExpressionNode, Node, StatementNode},
    builtin::BUILTINS,
    code::{make::make, Instructions, Opcode},
    object::Object,
    tokens::token::Token,
};

use self::{symbol_stack::SymbolTable as foo, symbol_table::{Symbol, SymbolTable}};

pub struct CompilerScope {
    pub instructions: Instructions,

    pub previous_instruction: (Opcode, usize),
    pub last_instruction: (Opcode, usize),
}

impl CompilerScope {
    fn new() -> CompilerScope {
        CompilerScope {
            instructions: Instructions(vec![]),
            previous_instruction: (Opcode::OpNoop, 0),
            last_instruction: (Opcode::OpNoop, 0),
        }
    }
}

pub struct Compiler {
    constants: Vec<Object>,
    symbol_table: SymbolTable,
    scopes: Vec<CompilerScope>,
}

pub struct Bytecode {
    pub instructions: Instructions,
    pub constants: Vec<Object>,
}

impl Bytecode {
    pub fn empty() -> Bytecode {
        Bytecode {
            instructions: Instructions(vec![]),
            constants: vec![],
        }
    }
}

type R = Result<(), String>;

impl Compiler {
    pub fn new() -> Compiler {
        let _ = foo::new();
        let mut c = Compiler {
            constants: vec![],
            symbol_table: SymbolTable::new(),
            scopes: vec![CompilerScope::new()],
        };

        for (idx, (name, _)) in BUILTINS.iter().enumerate() {
            c.symbol_table.define_builtin(idx, name);
        }

        c
    }

    pub fn new_from(self) -> Compiler {
        Compiler {
            constants: self.constants,
            symbol_table: self.symbol_table,
            scopes: vec![CompilerScope::new()],
        }
    }

    pub fn compile(&mut self, node: Node) -> R {
        match node {
            Node::Statement(_) => todo!(),
            Node::Expression(_) => todo!(),
            Node::Program(node) => self.compile_statements(&node.statements),
        }
    }

    fn scope(&self) -> &CompilerScope {
        self.scopes.last().unwrap()
    }

    fn scope_mut(&mut self) -> &mut CompilerScope {
        self.scopes.last_mut().unwrap()
    }

    fn compile_statements(&mut self, statements: &[StatementNode]) -> R {
        for statement in statements {
            self.compile_statement(statement)?;
        }

        Ok(())
    }

    fn load_symbol(&mut self, symbol: &Symbol)  {
        match symbol.scope {
                    symbol_table::Scope::Global => {
                        self.emit(Opcode::OpGetGlobal, vec![symbol.index])
                    }
                    symbol_table::Scope::Local => self.emit(Opcode::OpGetLocal, vec![symbol.index]),
                    symbol_table::Scope::Builtin => {
                        self.emit(Opcode::OpGetBuiltin, vec![symbol.index])
                    }
                    symbol_table::Scope::Free => todo!(),
        };
    }

    fn compile_statement(&mut self, statement: &StatementNode) -> R {
        match statement {
            StatementNode::LetStatement(node) => {
                self.compile_expression(&node.value)?;

                self.symbol_table.define(&node.identifier.value);
                let symbol = self.symbol_table.resolve(&node.identifier.value).unwrap();

                match symbol.scope {
                    symbol_table::Scope::Global => {
                        self.emit(Opcode::OpSetGlobal, vec![symbol.index])
                    }
                    symbol_table::Scope::Local => self.emit(Opcode::OpSetLocal, vec![symbol.index]),
                    symbol_table::Scope::Builtin => {
                        self.emit(Opcode::OpGetBuiltin, vec![symbol.index])
                    }
                    symbol_table::Scope::Free => todo!(),
                };

                Ok(())
            }
            StatementNode::ReturnStatement(node) => {
                self.compile_expression(&node.return_value)?;

                self.emit(Opcode::OpReturnValue, vec![]);

                Ok(())
            }
            StatementNode::BlockStatement(node) => self.compile_statements(&node.statements),
            StatementNode::ExpressionStatement(node) => {
                self.compile_expression(&node.expression)?;
                self.emit(Opcode::OpPop, vec![]);

                Ok(())
            }
        }
    }

    fn enter_scope(&mut self) {
        let scope = CompilerScope::new();

        self.symbol_table.enclose();

        self.scopes.push(scope);
    }

    fn leave_scope(&mut self) -> Instructions {
        let scope = self.scopes.pop().unwrap();

        self.symbol_table.pop();

        scope.instructions
    }

    fn compile_expression(&mut self, expression: &ExpressionNode) -> R {
        match expression {
            ExpressionNode::Identifier(node) => {
                let Some(symbol) = self.symbol_table.resolve(&node.value) else {
                    return Err(format!("undefined variable {}", node.value));
                };

                match symbol.scope {
                    symbol_table::Scope::Global => {
                        self.emit(Opcode::OpGetGlobal, vec![symbol.index])
                    }
                    symbol_table::Scope::Local => self.emit(Opcode::OpGetLocal, vec![symbol.index]),
                    symbol_table::Scope::Builtin => {
                        self.emit(Opcode::OpGetBuiltin, vec![symbol.index])
                    }
                    symbol_table::Scope::Free => {
                        self.emit(Opcode::OpGetFree, vec![symbol.index])
                    }
                };

                Ok(())
            }
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
            ExpressionNode::StringLiteral(node) => {
                let obj = Object::String(node.value.to_string());
                let pos = self.add_constant(obj);
                self.emit(Opcode::OpConstant, vec![pos]);

                Ok(())
            }
            ExpressionNode::ArrayLiteral(node) => {
                for element in &node.expressions {
                    self.compile_expression(element)?;
                }

                self.emit(Opcode::OpArray, vec![node.expressions.len()]);

                Ok(())
            }
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

                if self.scope().last_instruction.0.is(&Opcode::OpPop) {
                    self.remove_last();
                }

                let jump_pos = self.emit(Opcode::OpJump, vec![9999]);

                let after_concequence_pos = self.scope().instructions.0.len();
                self.change_operand(jump_not_truthy_pos, after_concequence_pos);

                if let Some(alternative) = &node.alternative {
                    self.compile_statements(&alternative.statements)?;

                    if self.scope().last_instruction.0.is(&Opcode::OpPop) {
                        self.remove_last();
                    }
                } else {
                    self.emit(Opcode::OpNull, vec![]);
                }

                let after_alternative_pos = self.scope().instructions.0.len();
                self.change_operand(jump_pos, after_alternative_pos);

                Ok(())
            }
            ExpressionNode::FunctionExpression(node) => {
                self.enter_scope();

                for parameter in &node.parameters {
                    self.symbol_table.define(&parameter.value);
                }

                self.compile_statements(&node.body.statements)?;

                if self.scope().last_instruction.0.is(&Opcode::OpPop) {
                    self.replace_last_with_return();
                }
                if !self.scope().last_instruction.0.is(&Opcode::OpReturnValue) {
                    self.emit(Opcode::OpReturn, vec![]);
                }

                let scope = self.symbol_table.current.clone();
                let free_symbols = &scope.lock().unwrap().free_symbols;
                let num_locals = self.symbol_table.current.lock().unwrap().count;
                let instructions = self.leave_scope();

                for s in free_symbols {
                    self.load_symbol(s);
                }

                let compiled_fn =
                    Object::CompiledFunction(instructions, num_locals, node.parameters.len());

                let operand = self.add_constant(compiled_fn);

                self.emit(Opcode::OpClosure, vec![operand, free_symbols.len()]);

                Ok(())
            }
            ExpressionNode::CallExpression(node) => {
                self.compile_expression(&node.function)?;

                for argument in &node.arguments {
                    self.compile_expression(argument)?;
                }

                self.emit(Opcode::OpCall, vec![node.arguments.len()]);

                Ok(())
            }
            ExpressionNode::IndexExpresssion(node) => {
                self.compile_expression(&node.left)?;
                self.compile_expression(&node.right)?;

                self.emit(Opcode::OpIndex, vec![]);

                Ok(())
            }
            ExpressionNode::HashLiteral(node) => {
                for item in &node.map {
                    self.compile_expression(&item.0)?;
                    self.compile_expression(&item.1)?;
                }

                self.emit(Opcode::OpHash, vec![node.map.len() * 2]);

                Ok(())
            }
        }
    }

    fn emit(&mut self, op: Opcode, operands: Vec<usize>) -> usize {
        let instruction = make(op, &operands);
        let pos = self.add_instruction(instruction);

        self.set_last_instruction(op, pos);

        pos
    }

    fn replace_last_with_return(&mut self) {
        let pos = self.scope().last_instruction.1;

        self.replace_instruction(pos, make(Opcode::OpReturnValue, &[]));

        self.scope_mut().last_instruction.0 = Opcode::OpReturnValue;
    }

    fn replace_instruction(&mut self, pos: usize, instruction: Vec<u8>) {
        for (i, b) in instruction.into_iter().enumerate() {
            self.scope_mut().instructions.0[pos + i] = b;
        }
    }

    fn change_operand(&mut self, op_pos: usize, operand: usize) {
        let op: Opcode = self.scope().instructions.0[op_pos].into();

        let instruction = make(op, &[operand]);

        self.replace_instruction(op_pos, instruction);
    }

    fn remove_last(&mut self) {
        let r = ..self.scope().last_instruction.1;
        self.scope_mut().instructions.0 = self.scope().instructions.0[r].to_vec();

        self.scope_mut().last_instruction = self.scope().previous_instruction;
    }

    fn set_last_instruction(&mut self, op: Opcode, pos: usize) {
        let previous = self.scope().last_instruction;
        self.scope_mut().last_instruction = (op, pos);

        self.scope_mut().previous_instruction = previous;
    }

    fn add_instruction(&mut self, mut instruction: Vec<u8>) -> usize {
        let pos = self.scope().instructions.0.len();
        self.scope_mut().instructions.0.append(&mut instruction);
        pos
    }

    fn add_constant(&mut self, object: Object) -> usize {
        self.constants.push(object);

        self.constants.len() - 1
    }

    pub fn bytecode(&self) -> Bytecode {
        Bytecode {
            instructions: self.scope().instructions.clone(),
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
        code::{make::make, Instructions, Opcode},
        compiler::Compiler,
        object::{test::test_object, Object},
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
        make(Opcode::OpJumpNotTruthy, &[10]),
        make(Opcode::OpConstant, &[0]),
        make(Opcode::OpJump, &[11]),
        make(Opcode::OpNull, &[]),
        make(Opcode::OpPop, &[]),
        make(Opcode::OpConstant, &[1]),
        make(Opcode::OpPop, &[]),
    ])]
    #[case("if (true) {10} else {20}; 3333;", vec![10, 20, 3333], vec![
        make(Opcode::OpTrue, &[]),
        make(Opcode::OpJumpNotTruthy, &[10]),
        make(Opcode::OpConstant, &[0]),
        make(Opcode::OpJump, &[13]),
        make(Opcode::OpConstant, &[1]),
        make(Opcode::OpPop, &[]),
        make(Opcode::OpConstant, &[2]),
        make(Opcode::OpPop, &[]),
    ])]
    fn test_conditionals(
        #[case] input: &str,
        #[case] constants: Vec<i64>,
        #[case] instructions: Vec<Vec<u8>>,
    ) {
        test_compiler(input, constants, instructions)
    }

    #[rstest]
    #[case("let one = 1; let two = 2;", vec![1, 2], vec![
        make(Opcode::OpConstant, &[0]),
        make(Opcode::OpSetGlobal, &[0]),
        make(Opcode::OpConstant, &[1]),
        make(Opcode::OpSetGlobal, &[1]),
    ])]
    #[case("let one = 1; one;", vec![1], vec![
        make(Opcode::OpConstant, &[0]),
        make(Opcode::OpSetGlobal, &[0]),
        make(Opcode::OpGetGlobal, &[0]),
        make(Opcode::OpPop, &[]),
    ])]
    #[case("let one = 1; let two = one; two;", vec![1], vec![
        make(Opcode::OpConstant, &[0]),
        make(Opcode::OpSetGlobal, &[0]),
        make(Opcode::OpGetGlobal, &[0]),
        make(Opcode::OpSetGlobal, &[1]),
        make(Opcode::OpGetGlobal, &[1]),
        make(Opcode::OpPop, &[]),
    ])]
    fn test_global_let_statements(
        #[case] input: &str,
        #[case] constants: Vec<i64>,
        #[case] instructions: Vec<Vec<u8>>,
    ) {
        test_compiler(input, constants, instructions)
    }

    #[rstest]
    #[case("\"monkey\"", vec!["monkey"], vec![
        make(Opcode::OpConstant, &[0]),
        make(Opcode::OpPop, &[]),
    ])]
    #[case("\"mon\" + \"key\"", vec!["mon", "key"], vec![
        make(Opcode::OpConstant, &[0]),
        make(Opcode::OpConstant, &[1]),
        make(Opcode::OpAdd, &[]),
        make(Opcode::OpPop, &[]),
    ])]
    fn test_string_expression(
        #[case] input: &str,
        #[case] constants: Vec<&'static str>,
        #[case] instructions: Vec<Vec<u8>>,
    ) {
        test_compiler(input, constants, instructions)
    }

    #[rstest]
    #[case("[]", vec![], vec![
        make(Opcode::OpArray, &[0]),
        make(Opcode::OpPop, &[]),
    ])]
    #[case("[1,2,3]", vec![1,2,3], vec![
        make(Opcode::OpConstant, &[0]),
        make(Opcode::OpConstant, &[1]),
        make(Opcode::OpConstant, &[2]),
        make(Opcode::OpArray, &[3]),
        make(Opcode::OpPop, &[]),
    ])]
    #[case("[1 + 2, 3 - 4, 5 * 6]", vec![1, 2, 3, 4, 5, 6], vec![
        make(Opcode::OpConstant, &[0]),
        make(Opcode::OpConstant, &[1]),
        make(Opcode::OpAdd, &[]),

        make(Opcode::OpConstant, &[2]),
        make(Opcode::OpConstant, &[3]),
        make(Opcode::OpSub, &[]),

        make(Opcode::OpConstant, &[4]),
        make(Opcode::OpConstant, &[5]),
        make(Opcode::OpMul, &[]),

        make(Opcode::OpArray, &[3]),
        make(Opcode::OpPop, &[]),
    ])]
    fn test_array_literals(
        #[case] input: &str,
        #[case] constants: Vec<i64>,
        #[case] instructions: Vec<Vec<u8>>,
    ) {
        test_compiler(input, constants, instructions)
    }

    #[rstest]
    #[case("{}", vec![], vec![
        make(Opcode::OpHash, &[0]),
        make(Opcode::OpPop, &[]),
    ])]
    #[case("{1: 2, 3: 4, 5: 6}", vec![1, 2, 3, 4, 5, 6], vec![
        make(Opcode::OpConstant, &[0]),
        make(Opcode::OpConstant, &[1]),
        make(Opcode::OpConstant, &[2]),
        make(Opcode::OpConstant, &[3]),
        make(Opcode::OpConstant, &[4]),
        make(Opcode::OpConstant, &[5]),
        make(Opcode::OpHash, &[6]),
        make(Opcode::OpPop, &[]),
    ])]
    #[case("{1: 2 + 3, 4: 5 * 6}", vec![1, 2, 3, 4, 5, 6], vec![
        make(Opcode::OpConstant, &[0]),
        make(Opcode::OpConstant, &[1]),
        make(Opcode::OpConstant, &[2]),
        make(Opcode::OpAdd, &[]),

        make(Opcode::OpConstant, &[3]),
        make(Opcode::OpConstant, &[4]),
        make(Opcode::OpConstant, &[5]),
        make(Opcode::OpMul, &[]),

        make(Opcode::OpHash, &[4]),
        make(Opcode::OpPop, &[]),
    ])]
    fn test_hash_literal(
        #[case] input: &str,
        #[case] constants: Vec<i64>,
        #[case] instructions: Vec<Vec<u8>>,
    ) {
        test_compiler(input, constants, instructions)
    }

    #[rstest]
    #[case("[1, 2, 3][1 + 1]", vec![1, 2, 3, 1, 1], vec![
        make(Opcode::OpConstant, &[0]),
        make(Opcode::OpConstant, &[1]),
        make(Opcode::OpConstant, &[2]),
        make(Opcode::OpArray, &[3]),

        make(Opcode::OpConstant, &[3]),
        make(Opcode::OpConstant, &[4]),
        make(Opcode::OpAdd, &[]),

        make(Opcode::OpIndex, &[]),
        make(Opcode::OpPop, &[]),
    ])]
    #[case("{1: 2}[2 - 1]", vec![1, 2, 2, 1], vec![
        make(Opcode::OpConstant, &[0]),
        make(Opcode::OpConstant, &[1]),
        make(Opcode::OpHash, &[2]),

        make(Opcode::OpConstant, &[2]),
        make(Opcode::OpConstant, &[3]),
        make(Opcode::OpSub, &[]),

        make(Opcode::OpIndex, &[]),
        make(Opcode::OpPop, &[]),
    ])]
    fn test_index_expression(
        #[case] input: &str,
        #[case] constants: Vec<i64>,
        #[case] instructions: Vec<Vec<u8>>,
    ) {
        test_compiler(input, constants, instructions)
    }

    #[rstest]
    #[case("fn() {return 5+10}", vec![Object::Integer(5),Object::Integer(10), Object::CompiledFunction(Instructions(vec![
        make(Opcode::OpConstant, &[0]),
        make(Opcode::OpConstant, &[1]),
        make(Opcode::OpAdd, &[]),
        make(Opcode::OpReturnValue, &[]),
    ].into_iter().flatten().collect()), 0, 0)], vec![
        make(Opcode::OpClosure, &[2, 0]),
        make(Opcode::OpPop, &[]),
    ])]
    #[case("fn() {5+10}", vec![Object::Integer(5),Object::Integer(10), Object::CompiledFunction(Instructions(vec![
        make(Opcode::OpConstant, &[0]),
        make(Opcode::OpConstant, &[1]),
        make(Opcode::OpAdd, &[]),
        make(Opcode::OpReturnValue, &[]),
    ].into_iter().flatten().collect()), 0, 0)], vec![
        make(Opcode::OpClosure, &[2, 0]),
        make(Opcode::OpPop, &[]),
    ])]
    #[case("fn() {1;2}", vec![Object::Integer(1),Object::Integer(2), Object::CompiledFunction(Instructions(vec![
        make(Opcode::OpConstant, &[0]),
        make(Opcode::OpPop, &[]),
        make(Opcode::OpConstant, &[1]),
        make(Opcode::OpReturnValue, &[]),
    ].into_iter().flatten().collect()), 0, 0)], vec![
        make(Opcode::OpClosure, &[2, 0]),
        make(Opcode::OpPop, &[]),
    ])]
    #[case("fn() {}", vec![Object::CompiledFunction(Instructions(vec![
        make(Opcode::OpReturn, &[]),
    ].into_iter().flatten().collect()), 0, 0)], vec![
        make(Opcode::OpClosure, &[0, 0]),
        make(Opcode::OpPop, &[]),
    ])]
    fn test_functions(
        #[case] input: &str,
        #[case] constants: Vec<Object>,
        #[case] instructions: Vec<Vec<u8>>,
    ) {
        test_compiler(input, constants, instructions)
    }

    #[rstest]
    #[case("fn() {24}();", vec![Object::Integer(24), Object::CompiledFunction(Instructions(vec![
        make(Opcode::OpConstant, &[0]),
        make(Opcode::OpReturnValue, &[]),
    ].into_iter().flatten().collect()), 0, 0)], vec![
        make(Opcode::OpClosure, &[1, 0]),
        make(Opcode::OpCall, &[0]),
        make(Opcode::OpPop, &[]),
    ])]
    #[case("let noArg = fn() {24};
noArg();", vec![Object::Integer(24), Object::CompiledFunction(Instructions(vec![
        make(Opcode::OpConstant, &[0]),
        make(Opcode::OpReturnValue, &[]),
    ].into_iter().flatten().collect()), 0, 0)], vec![
        make(Opcode::OpClosure, &[1, 0]),
        make(Opcode::OpSetGlobal, &[0]),
        make(Opcode::OpGetGlobal, &[0]),
        make(Opcode::OpCall, &[0]),
        make(Opcode::OpPop, &[]),
    ])]
    #[case("let oneArg = fn(a) { a }; 
oneArg(24);", vec![Object::CompiledFunction(Instructions(vec![
        make(Opcode::OpGetLocal, &[0]),
        make(Opcode::OpReturnValue, &[]),
    ].into_iter().flatten().collect()), 1, 1), Object::Integer(24)], vec![
        make(Opcode::OpClosure, &[0, 0]),
        make(Opcode::OpSetGlobal, &[0]),
        make(Opcode::OpGetGlobal, &[0]),
        make(Opcode::OpConstant, &[1]),
        make(Opcode::OpCall, &[1]),
        make(Opcode::OpPop, &[]),
    ])]
    #[case("let manyArg = fn(a, b, c) {a; b; c;};
manyArg(24,25,26);", vec![Object::CompiledFunction(Instructions(vec![
           make(Opcode::OpGetLocal, &[0]),
           make(Opcode::OpPop, &[]),
           make(Opcode::OpGetLocal, &[1]),
           make(Opcode::OpPop, &[]),
           make(Opcode::OpGetLocal, &[2]),
           make(Opcode::OpReturnValue, &[]),
        ].into_iter().flatten().collect()), 3, 3),
         Object::Integer(24),
         Object::Integer(25),
         Object::Integer(26),
    ],
    vec![
        make(Opcode::OpClosure, &[0, 0]),
        make(Opcode::OpSetGlobal, &[0]),
        make(Opcode::OpGetGlobal, &[0]),
        make(Opcode::OpConstant, &[1]),
        make(Opcode::OpConstant, &[2]),
        make(Opcode::OpConstant, &[3]),
        make(Opcode::OpCall, &[3]),
        make(Opcode::OpPop, &[]),
    ])]
    fn test_function_calls(
        #[case] input: &str,
        #[case] constants: Vec<Object>,
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

    #[test]
    fn compiler_scopes() {
        let mut compiler = Compiler::new();

        compiler.emit(Opcode::OpMul, vec![]);

        compiler.enter_scope();

        compiler.emit(Opcode::OpSub, vec![]);

        assert_eq!(compiler.scope().instructions.0.len(), 1);

        assert_eq!(compiler.scope().last_instruction.0, Opcode::OpSub);

        compiler.leave_scope();

        compiler.emit(Opcode::OpAdd, vec![]);
        assert_eq!(compiler.scope().instructions.0.len(), 2);

        assert_eq!(compiler.scope().last_instruction.0, Opcode::OpAdd);
        assert_eq!(compiler.scope().previous_instruction.0, Opcode::OpMul);
    }

    #[rstest]
    #[case("
let num = 55;
fn() {num}
", vec![Object::Integer(55), Object::CompiledFunction(Instructions(vec![
        make(Opcode::OpGetGlobal, &[0]),
        make(Opcode::OpReturnValue, &[]),
    ].into_iter().flatten().collect()), 0, 0)], vec![
        make(Opcode::OpConstant, &[0]),
        make(Opcode::OpSetGlobal, &[0]),
        make(Opcode::OpClosure, &[1, 0]),
        make(Opcode::OpPop, &[]),
    ])]
    #[case("
fn() {
    let num = 55;
    num
}
", vec![Object::Integer(55), Object::CompiledFunction(Instructions(vec![
        make(Opcode::OpConstant, &[0]),
        make(Opcode::OpSetLocal, &[0]),
        make(Opcode::OpGetLocal, &[0]),
        make(Opcode::OpReturnValue, &[]),
    ].into_iter().flatten().collect()), 1, 0)], vec![
        make(Opcode::OpClosure, &[1, 0]),
        make(Opcode::OpPop, &[]),
    ])]
    #[case("
fn() {
    let a = 55;
    let b = 77;
    a + b
}
", vec![Object::Integer(55),Object::Integer(77), Object::CompiledFunction(Instructions(vec![
        make(Opcode::OpConstant, &[0]),
        make(Opcode::OpSetLocal, &[0]),
        make(Opcode::OpConstant, &[1]),
        make(Opcode::OpSetLocal, &[1]),
        make(Opcode::OpGetLocal, &[0]),
        make(Opcode::OpGetLocal, &[1]),
        make(Opcode::OpAdd, &[]),
        make(Opcode::OpReturnValue, &[]),
    ].into_iter().flatten().collect()), 2, 0)], vec![
        make(Opcode::OpClosure, &[2, 0]),
        make(Opcode::OpPop, &[]),
    ])]
    fn test_let_statement_scopes(
        #[case] input: &str,
        #[case] constants: Vec<Object>,
        #[case] instructions: Vec<Vec<u8>>,
    ) {
        test_compiler(input, constants, instructions)
    }

    #[rstest]
    #[case("
len([]);
push([], 1);
", vec![Object::Integer(1)], vec![
        make(Opcode::OpGetBuiltin, &[0]),
        make(Opcode::OpArray, &[0]),
        make(Opcode::OpCall, &[1]),
        make(Opcode::OpPop, &[]),
        make(Opcode::OpGetBuiltin, &[5]),
        make(Opcode::OpArray, &[0]),
        make(Opcode::OpConstant, &[0]),
        make(Opcode::OpCall, &[2]),
        make(Opcode::OpPop, &[]),
    ])]
    #[case("
fn() { len([]) }
", vec![Object::CompiledFunction(Instructions(vec![
        make(Opcode::OpGetBuiltin, &[0]),
        make(Opcode::OpArray, &[0]),
        make(Opcode::OpCall, &[1]),
        make(Opcode::OpReturnValue, &[]),
    ].into_iter().flatten().collect()), 0, 0)], vec![
        make(Opcode::OpClosure, &[0, 0]),
        make(Opcode::OpPop, &[]),
    ])]
    fn test_builtins(
        #[case] input: &str,
        #[case] constants: Vec<Object>,
        #[case] instructions: Vec<Vec<u8>>,
    ) {
        test_compiler(input, constants, instructions)
    }

    #[rstest]
    #[case("
fn (a) {
    fn(b) {
        a + b
    }
}
", vec![
    Object::CompiledFunction(Instructions(vec![
        make(Opcode::OpGetFree, &[0]),
        make(Opcode::OpGetLocal, &[0]),
        make(Opcode::OpAdd, &[]),
        make(Opcode::OpReturnValue, &[]),
    ].into_iter().flatten().collect()), 1, 1),
    Object::CompiledFunction(Instructions(vec![
        make(Opcode::OpGetLocal, &[0]),
        make(Opcode::OpClosure, &[0, 1]),
        make(Opcode::OpReturnValue, &[]),
    ].into_iter().flatten().collect()), 1, 1)
    ], 
    vec![
        make(Opcode::OpClosure, &[1, 0]),
        make(Opcode::OpPop, &[]),
    ])]
    #[case("
fn(a) {
    fn(b) {
        fn(c) {
            a + b + c
        }
    }
}
", vec![
    Object::CompiledFunction(Instructions(vec![
        make(Opcode::OpGetFree, &[0]),
        make(Opcode::OpGetFree, &[1]),
        make(Opcode::OpAdd, &[]),
        make(Opcode::OpGetLocal, &[0]),
        make(Opcode::OpAdd, &[]),
        make(Opcode::OpReturnValue, &[]),
    ].into_iter().flatten().collect()), 1, 1),
    Object::CompiledFunction(Instructions(vec![
        make(Opcode::OpGetFree, &[0]),
        make(Opcode::OpGetLocal, &[0]),
        make(Opcode::OpClosure, &[0, 2]),
        make(Opcode::OpReturnValue, &[]),
    ].into_iter().flatten().collect()), 1, 1),
    Object::CompiledFunction(Instructions(vec![
        make(Opcode::OpGetLocal, &[0]),
        make(Opcode::OpClosure, &[1, 1]),
        make(Opcode::OpReturnValue, &[]),
    ].into_iter().flatten().collect()), 1, 1)
    ], 
    vec![
        make(Opcode::OpClosure, &[2, 0]),
        make(Opcode::OpPop, &[]),
    ])]

    fn test_closures(
        #[case] input: &str,
        #[case] constants: Vec<Object>,
        #[case] instructions: Vec<Vec<u8>>,
    ) {
        test_compiler(input, constants, instructions)
    }
}
