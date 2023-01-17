
#[derive(Debug, Clone)]
pub struct Token {
    kind: TokenKind,
    lexeme: String,
    line: u32,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, line: u32) -> Self {
        Self {
            kind, lexeme, line
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
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
    start: u32,
    current: u32,
    line: u32,
    tokens: Vec<Token>,
}

impl <'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            line: 1,
            tokens: Vec::new(),
        }
    }

    fn has_ended(&self) -> bool {
        self.current >= self.source.len() as u32
    }

    fn push_token(&mut self, kind: TokenKind) {
        let lexeme = &self.source[self.start as usize..self.current as usize];
        self.tokens.push(Token::new(kind, lexeme.to_string(), self.line));
    }

    fn scan_single_token(&mut self) {
        if let Some(c) = self.source.chars().nth(self.current as usize) {
            match c {
                '(' => self.push_token(TokenKind::LEFT_PAREN), 
                ')' => self.push_token(TokenKind::RIGHT_PAREN), 
                '[' => self.push_token(TokenKind::LEFT_BRACE), 
                ']' => self.push_token(TokenKind::RIGHT_BRACE), 
                '.' => self.push_token(TokenKind::DOT), 
                ',' => self.push_token(TokenKind::COMMA), 
                ';' => self.push_token(TokenKind::SEMICOLON), 
                '+' => self.push_token(TokenKind::PLUS), 
                '-' => self.push_token(TokenKind::MINUS), 
                '*' => self.push_token(TokenKind::STAR), 
                _ => todo!("scanner not implemented")
            }
        }

        self.current += 1;
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.has_ended() {
            self.start = self.current;
            self.scan_single_token();
        }
        self.tokens.push(Token::new(TokenKind::EOF, String::new(), self.line));
        self.tokens.clone()
    }
}
