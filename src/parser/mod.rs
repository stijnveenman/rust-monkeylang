use crate::{
    ast::{
        array_literal::ArrayLiteral, block_statement::BlockStatement,
        boolean_literal::BooleanLiteral, call_expression::CallExpression,
        expression_statement::ExpressionStatement, function_expression::FunctionExpression,
        grouped_expression::GroupedExpression, hash_literal::HashLiteral, identifier::Identifier,
        if_expression::IfExpression, index_expression::IndexExpression,
        infix_expression::InfixExpression, integer_literal::IntegerLiteral,
        let_statement::LetStatement, prefix_expression::PrefixExpression, program::Program,
        return_statement::ReturnStatement, string_literal::StringLiteral, ExpressionNode,
        ParsableResult, ParseInfix, ParsePrefix, ParseStatement, PrefixParser, StatementNode,
    },
    tokens::{lexer::Lexer, token::Token},
};

use self::precedence::{IntoPrecedence, Precedence};

pub mod precedence;

pub struct Parser {
    lexer: Lexer,
    pub current_token: Token,
    pub peek_token: Token,
}

impl Parser {
    pub fn new(input: String) -> Parser {
        let mut lexer = Lexer::new(input);

        let current_token = lexer.next_token();
        let next_token = lexer.next_token();

        Parser {
            lexer,
            current_token,
            peek_token: next_token,
        }
    }

    pub fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn expect_token(&mut self, token: Token) -> Result<(), String> {
        if self.peek_token.is(&token) {
            self.next_token();
            Ok(())
        } else {
            Err(format!(
                "invalid token, expected '{:?}' got '{:?}'",
                token, self.peek_token,
            ))
        }
    }

    fn parse_statement(&mut self) -> Option<ParsableResult<StatementNode>> {
        match self.current_token {
            Token::LET => Some(LetStatement::parse(self)),
            Token::RETURN => Some(ReturnStatement::parse(self)),
            _ => Some(ExpressionStatement::parse(self)),
        }
    }

    pub fn parse_prefix(&mut self) -> ParsableResult<ExpressionNode> {
        match self.current_token.clone() {
            Token::IDENT(_) => Identifier::parse_prefix(self),
            Token::INT(_) => IntegerLiteral::parse_prefix(self),
            Token::TRUE | Token::FALSE => BooleanLiteral::parse_prefix(self),
            Token::BANG | Token::MINUS => PrefixExpression::parse_prefix(self),
            Token::LPAREN => GroupedExpression::parse_prefix(self),
            Token::IF => IfExpression::parse_prefix(self),
            Token::FUNCTION => FunctionExpression::parse_prefix(self),
            Token::STRING(_) => StringLiteral::parse_prefix(self),
            Token::LBRACKET => ArrayLiteral::parse_prefix(self),
            Token::LBRACE => HashLiteral::parse_prefix(self),
            e => Err(format!("Invalid prefix token {:?}", e)),
        }
    }

    fn get_parse_infix(&mut self) -> Option<PrefixParser> {
        match self.peek_token.clone() {
            Token::PLUS
            | Token::MINUS
            | Token::SLASH
            | Token::ASTERISK
            | Token::EQ
            | Token::NOT_EQ
            | Token::LT
            | Token::GT => Some(InfixExpression::parse_infix),
            Token::LPAREN => Some(CallExpression::parse_infix),
            Token::LBRACKET => Some(IndexExpression::parse_infix),
            _ => None,
        }
    }

    pub fn parse_expression_list(
        &mut self,
        end_token: Token,
    ) -> ParsableResult<Vec<ExpressionNode>> {
        let mut arguments = vec![];
        if self.peek_token.is(&end_token) {
            self.next_token();
            return Ok(arguments);
        }

        self.next_token();

        loop {
            let expression = self.parse_expression(Precedence::LOWEST)?;
            arguments.push(expression);

            if !self.peek_token.is(&Token::COMMA) {
                break;
            }

            self.next_token();
            self.next_token();
        }

        self.expect_token(end_token)?;

        Ok(arguments)
    }

    pub fn parse_expression(&mut self, precedence: Precedence) -> ParsableResult<ExpressionNode> {
        let mut left = self.parse_prefix()?;

        while !self.peek_token.is(&Token::SEMICOLON) && precedence < self.peek_token.precedence() {
            let Some(parser) = self.get_parse_infix() else {
                return Ok(left);
            };

            self.next_token();

            left = parser(self, left)?;
        }

        Ok(left)
    }

    pub fn parse_program(&mut self) -> (Program, Vec<String>) {
        let mut program = Program { statements: vec![] };
        let mut errors = vec![];

        while self.current_token != Token::EOF {
            if let Some(result) = self.parse_statement() {
                match result {
                    Ok(statement) => program.statements.push(statement),
                    Err(e) => errors.push(e),
                }
            }
            self.next_token();
        }

        (program, errors)
    }

    pub fn parse_block(&mut self) -> ParsableResult<BlockStatement> {
        let mut block = BlockStatement {
            token: self.current_token.clone(),
            statements: vec![],
        };
        let mut errors = vec![];
        self.next_token();

        while !self.current_token.is(&Token::EOF) && !self.current_token.is(&Token::RBRACE) {
            if let Some(result) = self.parse_statement() {
                match result {
                    Ok(statement) => block.statements.push(statement),
                    Err(e) => errors.push(e),
                }
            }
            self.next_token();
        }

        if let Some(error) = errors.into_iter().next() {
            Err(error)
        } else {
            Ok(block)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        ast::{
            identifier::Identifier, let_statement::LetStatement, program::Program, AstNode,
            ExpressionNode, StatementNode,
        },
        tokens::token::Token,
    };

    #[test]
    fn ast_to_string() {
        let program = Program {
            statements: vec![StatementNode::LetStatement(LetStatement {
                token: Token::LET,
                identifier: Identifier {
                    token: Token::IDENT("myVar".into()),
                    value: "myVar".into(),
                },
                value: ExpressionNode::Identifier(Identifier {
                    token: Token::IDENT("anotherVar".into()),
                    value: "anotherVar".into(),
                }),
            })],
        };

        assert_eq!(program.string(), "let myVar = anotherVar;")
    }
}
