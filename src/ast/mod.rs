use crate::{parser::Parser, tokens::token::Token};

use self::{
    array_literal::ArrayLiteral, block_statement::BlockStatement, boolean_literal::BooleanLiteral,
    call_expression::CallExpression, expression_statement::ExpressionStatement,
    function_expression::FunctionExpression, hash_literal::HashLiteral, identifier::Identifier,
    if_expression::IfExpression, index_expression::IndexExpression,
    infix_expression::InfixExpression, integer_literal::IntegerLiteral,
    let_statement::LetStatement, prefix_expression::PrefixExpression, program::Program,
    return_statement::ReturnStatement, string_literal::StringLiteral,
};

pub mod array_literal;
pub mod block_statement;
pub mod boolean_literal;
pub mod call_expression;
pub mod expression_statement;
pub mod function_expression;
pub mod grouped_expression;
pub mod hash_literal;
pub mod identifier;
pub mod if_expression;
pub mod index_expression;
pub mod infix_expression;
pub mod integer_literal;
pub mod let_statement;
pub mod prefix_expression;
pub mod program;
pub mod return_statement;
pub mod string_literal;

pub trait AstNode {
    fn token(&self) -> &Token;
    fn string(&self) -> String;
}

#[derive(Debug, Clone)]
pub enum ExpressionNode {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    BooleanLiteral(BooleanLiteral),
    StringLiteral(StringLiteral),
    ArrayLiteral(ArrayLiteral),
    PrefixExpression(PrefixExpression),
    InfixExpression(InfixExpression),
    IfExpression(IfExpression),
    FunctionExpression(FunctionExpression),
    CallExpression(CallExpression),
    IndexExpresssion(IndexExpression),
    HashLiteral(HashLiteral),
}

#[derive(Debug, Clone)]
pub enum StatementNode {
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
    BlockStatement(BlockStatement),
    ExpressionStatement(ExpressionStatement),
}

#[derive(Debug, Clone)]
pub enum Node<'a> {
    Statement(&'a StatementNode),
    Expression(&'a ExpressionNode),
    Program(&'a Program),
}

impl<'a> From<&'a ExpressionNode> for Node<'a> {
    fn from(val: &'a ExpressionNode) -> Self {
        Node::Expression(val)
    }
}

impl<'a> From<&'a StatementNode> for Node<'a> {
    fn from(val: &'a StatementNode) -> Self {
        Node::Statement(val)
    }
}

impl<'a> From<&'a Program> for Node<'a> {
    fn from(val: &'a Program) -> Self {
        Node::Program(val)
    }
}

pub type ParsableResult<T> = Result<T, String>;

pub trait ParseStatement {
    fn parse(parser: &mut Parser) -> ParsableResult<StatementNode>;
}

pub trait ParsePrefix {
    fn parse_prefix(parser: &mut Parser) -> ParsableResult<ExpressionNode>;
}

pub type PrefixParser = fn(&mut Parser, ExpressionNode) -> ParsableResult<ExpressionNode>;
pub trait ParseInfix {
    fn parse_infix(parser: &mut Parser, left: ExpressionNode) -> ParsableResult<ExpressionNode>;
}

impl AstNode for StatementNode {
    fn token(&self) -> &Token {
        match self {
            StatementNode::LetStatement(i) => i.token(),
            StatementNode::ReturnStatement(i) => i.token(),
            StatementNode::ExpressionStatement(i) => i.token(),
            StatementNode::BlockStatement(i) => i.token(),
        }
    }

    fn string(&self) -> String {
        match self {
            StatementNode::LetStatement(i) => i.string(),
            StatementNode::ReturnStatement(i) => i.string(),
            StatementNode::ExpressionStatement(i) => i.string(),
            StatementNode::BlockStatement(i) => i.string(),
        }
    }
}

impl AstNode for ExpressionNode {
    fn token(&self) -> &Token {
        match self {
            ExpressionNode::Identifier(i) => i.token(),
            ExpressionNode::IntegerLiteral(i) => i.token(),
            ExpressionNode::PrefixExpression(i) => i.token(),
            ExpressionNode::InfixExpression(i) => i.token(),
            ExpressionNode::BooleanLiteral(i) => i.token(),
            ExpressionNode::IfExpression(i) => i.token(),
            ExpressionNode::FunctionExpression(i) => i.token(),
            ExpressionNode::CallExpression(i) => i.token(),
            ExpressionNode::StringLiteral(i) => i.token(),
            ExpressionNode::ArrayLiteral(i) => i.token(),
            ExpressionNode::IndexExpresssion(i) => i.token(),
            ExpressionNode::HashLiteral(i) => i.token(),
        }
    }

    fn string(&self) -> String {
        match self {
            ExpressionNode::Identifier(i) => i.string(),
            ExpressionNode::IntegerLiteral(i) => i.string(),
            ExpressionNode::PrefixExpression(i) => i.string(),
            ExpressionNode::InfixExpression(i) => i.string(),
            ExpressionNode::BooleanLiteral(i) => i.string(),
            ExpressionNode::IfExpression(i) => i.string(),
            ExpressionNode::FunctionExpression(i) => i.string(),
            ExpressionNode::CallExpression(i) => i.string(),
            ExpressionNode::StringLiteral(i) => i.string(),
            ExpressionNode::ArrayLiteral(i) => i.string(),
            ExpressionNode::IndexExpresssion(i) => i.string(),
            ExpressionNode::HashLiteral(i) => i.string(),
        }
    }
}

#[cfg(test)]
mod test {
    use std::any::Any;

    use super::ExpressionNode;

    pub fn test_expression<T: std::fmt::Debug + Any>(expression: &ExpressionNode, value: &T) {
        let value_any = value as &dyn Any;

        match expression {
            ExpressionNode::Identifier(v) => {
                assert_eq!(
                    v.value,
                    value_any.downcast_ref::<&str>().unwrap().to_string()
                )
            }
            ExpressionNode::IntegerLiteral(v) => {
                let val = value_any
                    .downcast_ref::<i64>()
                    .copied()
                    .or(value_any.downcast_ref::<i32>().map(|v| i64::from(*v)))
                    .unwrap();
                assert_eq!(&v.value, &val)
            }
            ExpressionNode::BooleanLiteral(v) => {
                assert_eq!(&v.value, value_any.downcast_ref().unwrap())
            }
            ExpressionNode::StringLiteral(v) => {
                assert_eq!(&v.value, value_any.downcast_ref::<&str>().unwrap())
            }
            _ => todo!(),
        }
    }
}
