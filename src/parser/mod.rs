use crate::tokens::lexer::Lexer;

struct Parser {
    lexer: Lexer,
    statements: Vec<()>,
}

impl Parser {
    fn new(input: String) -> Parser {
        Parser {
            lexer: Lexer::new(input),
            statements: vec![],
        }
    }
}

#[test]
fn test_basic_parser() {
    let input = "
let x = 5;
let y = 10;
let foobar = 838383;
";
    let parser = Parser::new(input.into());

    //let program = parser.parse_program();
    
    //assert_eq!(program.statements, 3);
}

