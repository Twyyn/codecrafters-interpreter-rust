use std::fmt;

#[rustfmt::skip] #[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    LEFT_PAREN, RIGHT_PAREN,
    LEFT_BRACE, RIGHT_BRACE,

    COMMA, DOT, SEMICOLON,
    MINUS, PLUS, SLASH, STAR,
    GREATER, LESS, EQUAL, BANG,

    GREATER_EQUAL, LESS_EQUAL,
    EQUAL_EQUAL, BANG_EQUAL,

    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<String>,
    pub line: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let literal_str = self.literal.as_deref().unwrap_or("null");
        write!(f, "{:?} {} {}", self.token_type, self.lexeme, literal_str)
    }
}

#[derive(Debug)]
pub struct Lexer {
    source: Vec<char>,
    tokens: Vec<Token>,
    current: usize,
    start: usize,
    line: usize,
    had_error: bool,
}
impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: Vec::new(),
            current: 0,
            start: 0,
            line: 1,
            had_error: false,
        }
    }

    pub fn scan_tokens(&mut self) -> &[Token] {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.eof_token();
        &self.tokens
    }

    /* Helpers */
    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LEFT_PAREN, None),
            ')' => self.add_token(TokenType::RIGHT_PAREN, None),
            '{' => self.add_token(TokenType::LEFT_BRACE, None),
            '}' => self.add_token(TokenType::RIGHT_BRACE, None),
            ',' => self.add_token(TokenType::COMMA, None),
            '.' => self.add_token(TokenType::DOT, None),
            ';' => self.add_token(TokenType::SEMICOLON, None),
            '-' => self.add_token(TokenType::MINUS, None),
            '+' => self.add_token(TokenType::PLUS, None),
            '/' => self.add_token(TokenType::SLASH, None),
            '*' => self.add_token(TokenType::STAR, None),
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GREATER_EQUAL, None)
                } else {
                    self.add_token(TokenType::GREATER, None)
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LESS_EQUAL, None)
                } else {
                    self.add_token(TokenType::LESS, None)
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EQUAL_EQUAL, None)
                } else {
                    self.add_token(TokenType::EQUAL, None)
                }
            }
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BANG_EQUAL, None)
                } else {
                    self.add_token(TokenType::BANG, None)
                }
            }

            
            _ => self.error(self.line, &format!("Unexpected character: {}", c)),
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<String>) {
        let lexeme = self.source[self.start..self.current]
            .iter()
            .collect::<String>();

        self.tokens.push(Token {
            token_type,
            lexeme,
            literal,
            line: self.line,
        });
    }

    fn eof_token(&mut self) {
        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: String::new(),
            literal: None,
            line: self.line,
        });
    }

    fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: usize, where_msg: &str, message: &str) {
        eprintln!("[line {}] Error:{} {}", line, where_msg, message);
        self.had_error = true;
    }

    pub fn had_error(&self) -> bool {
        self.had_error
    }
}
