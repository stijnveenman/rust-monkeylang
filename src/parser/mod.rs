use crate::{
    ast::{
        let_statement::LetStatement, program::Program, return_statement::ReturnStatement,
        ParsableResult, ParseStatement, StatementNode,
    },
    tokens::{lexer::Lexer, token::Token},
};

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

    pub fn next_token(&mut self) -> Token {
        let current = self.current_token.clone();
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
        current
    }

    pub fn expect_token(&mut self, token: Token) -> Result<Token, String> {
        if self.peek_token.is(&token) {
            Ok(self.next_token())
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
            _ => None,
        }
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
}
