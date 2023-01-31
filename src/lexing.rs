
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

    fn next_is(&mut self, c: char) -> bool {
        if self.has_ended() {
            return false;
        }

        if let Some(symbol) = self.source.chars().nth((self.current + 1) as usize) {
            if symbol == c {
                self.current += 1;
                return true;
            }
        }
        false
    }

    fn peek(&self) -> Option<char> {
        return self.source.chars().nth((self.current + 1) as usize)
    }
    
    fn peek_next(&self) -> Option<char> {
        return self.source.chars().nth((self.current + 2) as usize)
    }

    fn push_token(&mut self, kind: TokenKind) {
        let lexeme = &self.source[self.start as usize..(self.current+1) as usize];
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
                '\n' => self.line += 1,
                '/' => {
                    if self.next_is('/') {
                        while self.peek().unwrap() != '\n' && !self.has_ended() {
                            self.current += 1;
                        }
                    } else {
                        self.push_token(TokenKind::SLASH);
                    }
                },
                '=' => {
                    let kind = if self.next_is('=') { TokenKind::EQUAL_EQUAL } else { TokenKind::EQUAL };
                    self.push_token(kind);
                },
                '!' => {
                    let kind = if self.next_is('=') { TokenKind::BANG_EQUAL } else { TokenKind::BANG };
                    self.push_token(kind);
                },
                '>' => {
                    let kind = if self.next_is('=') { TokenKind::GREATER_EQUAL } else { TokenKind::GREATER };
                    self.push_token(kind);
                },
                '<' => {
                    let kind = if self.next_is('=') { TokenKind::LESS_EQUAL } else { TokenKind::LESS };
                    self.push_token(kind);
                },
                '"' => {
                    while !self.next_is('"') && !self.has_ended() {
                        if self.peek() == Some('\n') {
                            self.line += 1;
                        }
                        self.current += 1;
                    }
                    if self.has_ended() {
                        // @TODO: use better error handling
                        panic!("string with unclosed delimeters");
                    }
                    // trick to trim " from the string
                    self.start += 1;
                    self.current -= 1;
                    self.push_token(TokenKind::STRING);

                    // we need to increment again so we don't process the same char again
                    self.current += 1;
                },
                ' ' | '\r' | '\t' => {},
                _ => {
                    if c.is_ascii_digit() {
                        while self.peek().is_some() && self.peek().unwrap().is_ascii_digit() {
                            self.current += 1;
                        }
                        if self.peek().is_some() && self.peek().unwrap() == '.' {
                            if self.peek_next().is_some() && self.peek_next().unwrap().is_ascii_digit() {
                                self.current += 1;
                                while self.peek().is_some() && self.peek().unwrap().is_ascii_digit() {
                                    self.current += 1;
                                }
                            } else {
                                // @TODO: use better error handling
                                panic!("wrong number format");
                            }
                        }
                        self.push_token(TokenKind::NUMBER) 
                    } else {
                        todo!("scanner not implemented for {}", c);
                    }
                }
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
