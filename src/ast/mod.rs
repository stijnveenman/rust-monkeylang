use crate::{parser::Parser, tokens::token::Token};

use self::{identifier::Identifier, let_statement::LetStatement};

pub mod identifier;
pub mod let_statement;
pub mod program;

pub trait Statement {
    fn token(&self) -> &Token;
}

pub trait Expression {
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
    Placeolder,
}

pub type ParsableResult<T> = Result<T, String>;

pub trait ParseStatement {
    fn parse(parser: &mut Parser) -> ParsableResult<StatementNode>;
}
