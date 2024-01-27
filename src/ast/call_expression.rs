use crate::{
    parser::{precedence::Precedence, Parser},
    tokens::token::Token,
};

use super::{AstNode, ExpressionNode, ParsableResult, ParseInfix};

#[derive(Debug)]
pub struct CallExpression {
    pub token: Token,
    pub function: Box<ExpressionNode>,
    pub arguments: Vec<ExpressionNode>,
}

impl AstNode for CallExpression {
    fn token(&self) -> &Token {
        &self.token
    }

    fn string(&self) -> String {
        format!(
            "{} ({})",
            self.function.string(),
            self.arguments
                .iter()
                .map(|a| a.string())
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

impl CallExpression {
    fn parse_arguments(parser: &mut Parser) -> ParsableResult<Vec<ExpressionNode>> {
        let mut arguments = vec![];
        if parser.peek_token.is(&Token::RPAREN) {
            parser.next_token();
            return Ok(arguments);
        }

        parser.next_token();

        loop {
            let expression = parser.parse_expression(Precedence::LOWEST)?;
            arguments.push(expression);

            if !parser.peek_token.is(&Token::COMMA) {
                break;
            }

            parser.next_token();
            parser.next_token();
        }

        parser.expect_token(Token::RPAREN)?;

        Ok(arguments)
    }
}

impl ParseInfix for CallExpression {
    fn parse_infix(parser: &mut Parser, left: ExpressionNode) -> ParsableResult<ExpressionNode> {
        let token = parser.current_token.clone();

        let arguments = CallExpression::parse_arguments(parser)?;

        Ok(ExpressionNode::CallExpression(CallExpression {
            token,
            function: Box::new(left),
            arguments,
        }))
    }
}

#[cfg(test)]
mod test {

    use rstest::rstest;

    use crate::{
        ast::{
            infix_expression::test::test_infix_expression, test::test_expression, AstNode,
            ExpressionNode, StatementNode,
        },
        parser::Parser,
        tokens::token::Token,
    };

    #[test]
    fn test_call_expression() {
        let input = "add(1, 2 * 3, 4 + 5);";
        let mut parser = Parser::new(input.into());

        let (program, errors) = parser.parse_program();
        let empty: Vec<String> = vec![];

        assert_eq!(errors, empty);
        assert_eq!(program.statements.len(), 1);

        let node = program.statements.first().unwrap();
        let StatementNode::ExpressionStatement(expression) = node else {
            panic!("expected ExpressionStatement for node, got {:?}", node);
        };

        let ExpressionNode::CallExpression(expression) = &expression.expression else {
            panic!("expected CallExpression for node, got {:?}", node);
        };

        test_expression(&expression.function, &"add");

        test_expression(expression.arguments.first().unwrap(), &1u64);
        test_infix_expression(
            expression.arguments.get(1).unwrap(),
            2u64,
            Token::ASTERISK,
            3u64,
        );
        test_infix_expression(
            expression.arguments.get(2).unwrap(),
            4u64,
            Token::PLUS,
            5u64,
        );
    }

    #[rstest]
    #[case("add();", "add", vec![])]
    #[case("add(1);", "add", vec!["1"])]
    #[case("add(1, 2 * 3, 4 + 5);", "add", vec!["1", "(2 * 3)", "(4 + 5)"])]
    fn test_call_arguments(
        #[case] input: &str,
        #[case] name: &'static str,
        #[case] arguments: Vec<&str>,
    ) {
        let mut parser = Parser::new(input.into());

        let (program, errors) = parser.parse_program();
        let empty: Vec<String> = vec![];

        assert_eq!(errors, empty);
        assert_eq!(program.statements.len(), 1);

        let node = program.statements.first().unwrap();
        let StatementNode::ExpressionStatement(expression) = node else {
            panic!("expected ExpressionStatement for node, got {:?}", node);
        };

        let ExpressionNode::CallExpression(expression) = &expression.expression else {
            panic!("expected CallExpression for node, got {:?}", node);
        };

        test_expression(&expression.function, &name);

        assert_eq!(
            expression
                .arguments
                .iter()
                .map(|a| a.string())
                .collect::<Vec<_>>(),
            arguments
        );
    }
}
