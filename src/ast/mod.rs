use crate::{parser::Parser, tokens::token::Token};

use self::{
    expression_statement::ExpressionStatement, identifier::Identifier, let_statement::LetStatement,
    return_statement::ReturnStatement,
};

pub mod expression_statement;
pub mod identifier;
pub mod let_statement;
pub mod program;
pub mod return_statement;

pub trait AstNode {
    fn token(&self) -> &Token;
    fn string(&self) -> String;
}

#[derive(Debug)]
pub enum ExpressionNode {
    Identifier(Identifier),
    Placeolder,
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
