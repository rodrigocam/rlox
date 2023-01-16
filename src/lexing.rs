
#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    lexeme: String,
    line: u32,
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum TokenKind {
    LEFT_PAREN, RIGHT_PAREN,
    LEFT_BRACE, RIGHT_BRACE,

    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

    BANG, BANG_EQUAL,
    EQUAL, EQUAL_EQUAL,
    GREATER, GREATER_EQUAL,
    LESS, LESS_EQUAL,

    IDENTIFIER, STRING, NUMBER,

    AND, CLASS, ELSE, FALSE, FUN,
    FOR, IF, NIL, OR, PRINT, RETURN, SUPER,
    THIS, TRUE, VAR, WHILE,

    EOF
}

pub struct Scanner<'a> {
    source: &'a str,
}

impl <'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source
        }
    }

    pub fn scan_tokens(&self) -> Vec<Token> {
        vec![]
    }
}
