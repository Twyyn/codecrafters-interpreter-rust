use crate::token::{KEYWORDS, Token, TokenType};

#[derive(Debug)]
pub struct Lexer {
    source: Vec<char>,
    tokens: Vec<Token>,
    errors: Vec<LexError>,
    current: usize,
    start: usize,
    line: usize,
}

#[derive(Debug)]
pub struct LexError {
    pub line: usize,
    pub message: String,
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[line {}] Error: {}", self.line, self.message)
    }
}

impl std::error::Error for LexError {}

pub struct LexResult {
    pub tokens: Vec<Token>,
    pub errors: Vec<LexError>,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: Vec::new(),
            errors: Vec::new(),
            current: 0,
            start: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(mut self) -> LexResult {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: String::new(),
            literal: None,
            line: self.line,
        });

        LexResult {
            tokens: self.tokens,
            errors: self.errors,
        }
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LEFT_PAREN),
            ')' => self.add_token(TokenType::RIGHT_PAREN),
            '{' => self.add_token(TokenType::LEFT_BRACE),
            '}' => self.add_token(TokenType::RIGHT_BRACE),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            ';' => self.add_token(TokenType::SEMICOLON),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            '*' => self.add_token(TokenType::STAR),

            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::SLASH);
                }
            }

            '>' => self.add_comparison_token(TokenType::GREATER, TokenType::GREATER_EQUAL),
            '<' => self.add_comparison_token(TokenType::LESS, TokenType::LESS_EQUAL),
            '=' => self.add_comparison_token(TokenType::EQUAL, TokenType::EQUAL_EQUAL),
            '!' => self.add_comparison_token(TokenType::BANG, TokenType::BANG_EQUAL),

            '"' => self.string(),
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(),

            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,

            _ => self.error(format!("Unexpected character: {c}")),
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error("Unterminated string.".into());
            return;
        }

        self.advance(); // closing "

        let literal: String = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();

        self.add_token_with_literal(TokenType::STRING, literal);
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let lexeme = self.lexeme();
        let n: f64 = lexeme.parse().unwrap(); // valid by construction

        let literal = if n.fract() == 0.0 {
            format!("{n:.1}")
        } else {
            n.to_string()
        };

        self.add_token_with_literal(TokenType::NUMBER, literal);
    }

    fn identifier(&mut self) {
        while matches!(self.peek(), 'a'..='z' | 'A'..='Z' | '0'..='9' | '_') {
            self.advance();
        }

        let text = self.lexeme();
        let token_type = KEYWORDS
            .get(text.as_str())
            .copied()
            .unwrap_or(TokenType::IDENTIFIER);
        self.add_token(token_type);
    }

    // === Helpers ===

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token {
            token_type,
            lexeme: self.lexeme(),
            literal: None,
            line: self.line,
        });
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: String) {
        self.tokens.push(Token {
            token_type,
            lexeme: self.lexeme(),
            literal: Some(literal),
            line: self.line,
        });
    }

    fn add_comparison_token(&mut self, single: TokenType, double: TokenType) {
        let token_type = if self.match_char('=') { double } else { single };
        self.add_token(token_type);
    }

    fn lexeme(&self) -> String {
        self.source[self.start..self.current].iter().collect()
    }

    // === Navigation ===

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.peek() == expected {
            self.current += 1;
            true
        } else {
            false
        }
    }

    fn peek(&self) -> char {
        self.source.get(self.current).copied().unwrap_or('\0')
    }

    fn peek_next(&self) -> char {
        self.source.get(self.current + 1).copied().unwrap_or('\0')
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn error(&mut self, message: String) {
        self.errors.push(LexError {
            line: self.line,
            message,
        });
    }
}
