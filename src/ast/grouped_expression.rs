use crate::{parser::precedence::Precedence, tokens::token::Token};

use super::ParsePrefix;

pub struct GroupedExpression();

impl ParsePrefix for GroupedExpression {
    fn parse_prefix(
        parser: &mut crate::parser::Parser,
    ) -> super::ParsableResult<super::ExpressionNode> {
        parser.next_token();

        let expression = parser.parse_expression(Precedence::LOWEST)?;

        parser.expect_token(Token::RPAREN)?;

        Ok(expression)
    }
}
