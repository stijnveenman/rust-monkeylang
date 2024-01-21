#[derive(PartialEq, Eq, Debug)]
#[allow(non_camel_case_types)]
pub enum Token {
    ILLEGAL,
    EOF,

    // identifier + literals
    IDENT(String),
    INT(u64),

    // operators
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,

    LT,
    GT,

    EQ,
    NOT_EQ,

    // delmiters
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

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
}
