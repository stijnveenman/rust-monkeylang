#[derive(PartialEq, Eq, Debug)]
pub enum Token {
    ILLEGAL,
    EOF,

    IDENT(String),
    INT(u64),

    ASSIGN,
    PLUS,

    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    FUNCTION,
    LEYT,
}
