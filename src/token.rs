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

pub static KEYWORDS: LazyLock<HashMap<&str, TokenType>> = LazyLock::new(|| {
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
