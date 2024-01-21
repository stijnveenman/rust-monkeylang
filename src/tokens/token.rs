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
    LET,
}

impl Token {
    pub fn from_ident(ident: String) -> Token {
        match ident.as_str() {
            "let" => Token::LET,
            "fn" => Token::FUNCTION,
            _ => Token::IDENT(ident),
        }
    }
}
