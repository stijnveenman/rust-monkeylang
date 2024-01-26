use crate::{parser::Parser, tokens::token::Token};

use self::{
    boolean_literal::BooleanLiteral, expression_statement::ExpressionStatement,
    identifier::Identifier, infix_expression::InfixExpression, integer_literal::IntegerLiteral,
    let_statement::LetStatement, prefix_expression::PrefixExpression,
    return_statement::ReturnStatement,
};

pub mod boolean_literal;
pub mod expression_statement;
pub mod identifier;
pub mod infix_expression;
pub mod integer_literal;
pub mod let_statement;
pub mod prefix_expression;
pub mod program;
pub mod return_statement;

pub trait AstNode {
    fn token(&self) -> &Token;
    fn string(&self) -> String;
}

#[derive(Debug)]
pub enum ExpressionNode {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    BooleanLiteral(BooleanLiteral),
    PrefixExpression(PrefixExpression),
    InfixExpression(InfixExpression),
}

#[derive(Debug)]
pub enum StatementNode {
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
    ExpressionStatement(ExpressionStatement),
}

pub type ParsableResult<T> = Result<T, String>;

pub trait ParseStatement {
    fn parse(parser: &mut Parser) -> ParsableResult<StatementNode>;
}

pub trait ParsePrefix {
    fn parse_prefix(parser: &mut Parser) -> ParsableResult<ExpressionNode>;
}

pub trait ParseInfix {
    fn parse_infix(parser: &mut Parser, left: ExpressionNode) -> ParsableResult<ExpressionNode>;
}

impl AstNode for StatementNode {
    fn token(&self) -> &Token {
        match self {
            StatementNode::LetStatement(i) => i.token(),
            StatementNode::ReturnStatement(i) => i.token(),
            StatementNode::ExpressionStatement(i) => i.token(),
        }
    }

    fn string(&self) -> String {
        match self {
            StatementNode::LetStatement(i) => i.string(),
            StatementNode::ReturnStatement(i) => i.string(),
            StatementNode::ExpressionStatement(i) => i.string(),
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
        }
    }

    fn string(&self) -> String {
        match self {
            ExpressionNode::Identifier(i) => i.string(),
            ExpressionNode::IntegerLiteral(i) => i.string(),
            ExpressionNode::PrefixExpression(i) => i.string(),
            ExpressionNode::InfixExpression(i) => i.string(),
            ExpressionNode::BooleanLiteral(i) => i.string(),
        }
    }
}
