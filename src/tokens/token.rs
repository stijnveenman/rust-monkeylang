use std::mem::{self};

#[derive(PartialEq, Eq, Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum Token {
    ILLEGAL,
    EOF,

    // identifier + literals
    IDENT(String),
    INT(i64),
    STRING(String),

    // operators
    ASSIGN,
    BANG,
    PLUS,
    MINUS,
    ASTERISK,
    SLASH,

    LT,
    GT,

    EQ,
    NOT_EQ,

    // delmiters
    COMMA,
    SEMICOLON,
    COLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    LBRACKET,
    RBRACKET,

    // keywords
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

impl Token {
    pub fn from_ident(ident: String) -> Token {
        match ident.as_str() {
            "fn" => Token::FUNCTION,
            "let" => Token::LET,
            "true" => Token::TRUE,
            "false" => Token::FALSE,
            "if" => Token::IF,
            "else" => Token::ELSE,
            "return" => Token::RETURN,

            _ => Token::IDENT(ident),
        }
    }

    pub fn is(&self, other: &Token) -> bool {
        mem::discriminant(self) == mem::discriminant(other)
    }
}
