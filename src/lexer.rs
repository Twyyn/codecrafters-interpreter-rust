use std::collections::HashMap;
use std::fmt;
use std::sync::LazyLock;

#[rustfmt::skip] #[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    LEFT_PAREN, RIGHT_PAREN,
    LEFT_BRACE, RIGHT_BRACE,

    COMMA, DOT, SEMICOLON,
    MINUS, PLUS, SLASH, STAR,
    GREATER, LESS, EQUAL, BANG,

    GREATER_EQUAL, LESS_EQUAL,
    EQUAL_EQUAL, BANG_EQUAL,

    STRING, NUMBER, IDENTIFIER,

    CLASS, VAR, SUPER, PRINT, RETURN, THIS, AND, OR, 
    IF, ELSE, FALSE, TRUE, WHILE, FOR, FUN, NIL, 

    EOF,
}

static KEYWORDS: LazyLock<HashMap<&str, TokenType>> = LazyLock::new(|| {
    let mut keywords = HashMap::new();
    keywords.insert("class", TokenType::CLASS);
    keywords.insert("var", TokenType::VAR);
    keywords.insert("super", TokenType::SUPER);
    keywords.insert("print", TokenType::PRINT);
    keywords.insert("return", TokenType::RETURN);
    keywords.insert("this", TokenType::THIS);
    keywords.insert("and", TokenType::AND);
    keywords.insert("or", TokenType::OR);
    keywords.insert("if", TokenType::IF);
    keywords.insert("else", TokenType::ELSE);
    keywords.insert("true", TokenType::TRUE);
    keywords.insert("false", TokenType::FALSE);
    keywords.insert("while", TokenType::WHILE);
    keywords.insert("for", TokenType::FOR);
    keywords.insert("fun", TokenType::FUN);
    keywords.insert("nil", TokenType::NIL);
    keywords
});

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

    // === Core Scanning Methods ===

    pub fn scan_tokens(&mut self) -> &[Token] {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.eof_token();
        &self.tokens
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            // Single-character tokens
            '(' => self.add_token(TokenType::LEFT_PAREN, None),
            ')' => self.add_token(TokenType::RIGHT_PAREN, None),
            '{' => self.add_token(TokenType::LEFT_BRACE, None),
            '}' => self.add_token(TokenType::RIGHT_BRACE, None),
            ',' => self.add_token(TokenType::COMMA, None),
            '.' => self.add_token(TokenType::DOT, None),
            ';' => self.add_token(TokenType::SEMICOLON, None),
            '-' => self.add_token(TokenType::MINUS, None),
            '+' => self.add_token(TokenType::PLUS, None),
            '*' => self.add_token(TokenType::STAR, None),

            // One or two character tokens
            '/' => self.scan_slash(),
            '>' => self.scan_comparison(TokenType::GREATER, TokenType::GREATER_EQUAL),
            '<' => self.scan_comparison(TokenType::LESS, TokenType::LESS_EQUAL),
            '=' => self.scan_comparison(TokenType::EQUAL, TokenType::EQUAL_EQUAL),
            '!' => self.scan_comparison(TokenType::BANG, TokenType::BANG_EQUAL),

            // Literals
            '"' => self.string(),
            c if self.is_digit(c) => self.number(),
            c if self.is_alpha(c) => self.identifier(),

            // Whitespace
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,

            // Error
            _ => self.error(self.line, &format!("Unexpected character: {}", c)),
        }
    }

    fn scan_slash(&mut self) {
        if self.match_char('/') {
            while self.peek() != '\n' && !self.is_at_end() {
                self.advance();
            }
        } else {
            self.add_token(TokenType::SLASH, None);
        }
    }

    fn scan_comparison(&mut self, single: TokenType, double: TokenType) {
        let token_type = if self.match_char('=') { double } else { single };
        self.add_token(token_type, None);
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error(self.line, "Unterminated string.");
            return;
        }

        self.advance();

        let literal = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect::<String>();

        self.add_token(TokenType::STRING, Some(literal));
    }

    fn number(&mut self) {
        self.consume_digits();
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();
            self.consume_digits();
        }

        let value = self.get_lexeme();
        match value.parse::<f64>() {
            Ok(number) => {
                let literal = self.format_number(number);
                self.add_token(TokenType::NUMBER, Some(literal));
            }
            Err(_) => {
                self.error(self.line, &format!("Invalid number: {}", value));
            }
        }
    }

    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text = self.get_lexeme();

        let token_type = KEYWORDS
            .get(text.as_str())
            .unwrap_or(&TokenType::IDENTIFIER);

        self.add_token(*token_type, None);
    }

    fn is_alpha(&self, c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }

    // === Token Management ===

    fn get_lexeme(&self) -> String {
        self.source[self.start..self.current].iter().collect()
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<String>) {
        let lexeme = self.get_lexeme();
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

    // === Character Navigation ===

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source[self.current]
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source[self.current + 1]
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    // === Character Classification ===

    fn is_digit(&self, c: char) -> bool {
        c.is_ascii_digit()
    }

    fn consume_digits(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }
    }

    // === Formatting ===

    fn format_number(&self, number: f64) -> String {
        if number.fract() == 0.0 {
            format!("{:.1}", number)
        } else {
            number.to_string()
        }
    }

    // === Error Handling ===

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
