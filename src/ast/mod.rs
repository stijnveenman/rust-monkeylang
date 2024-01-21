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
