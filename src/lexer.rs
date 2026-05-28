use crate::{
    errors::InterpreterError,
    token::{Token, TokenKind},
};

#[derive(Debug)]
pub struct Lexer<'a> {
    cursor: Cursor<'a>,
    tokens: Vec<Token<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            cursor: Cursor::new(src),
            tokens: Vec::new(),
        }
    }

    pub fn scan_tokens(mut self) -> Result<Vec<Token<'a>>, InterpreterError> {
        while !self.cursor.is_at_end() {
            if let Some(c) = self.cursor.advance() {
                match c {
                    '(' => self.add_token(TokenKind::LeftParen, "("),
                    ')' => self.add_token(TokenKind::LeftParen, ")"),
                    '}' => self.add_token(TokenKind::LeftBracket, "}"),
                    '{' => self.add_token(TokenKind::RightBracket, "{"),

                    _ => {
                        return Err(InterpreterError::Lex {
                            line: self.cursor.line,
                            message: format!("Unexpected character: '{c}'"),
                        });
                    }
                }
            }
        }
        self.add_token(TokenKind::EOF, "");
        Ok(self.tokens)
    }

    fn add_token(&mut self, kind: TokenKind, lexeme: &'a str) {
        self.tokens
            .push(Token::new(kind, lexeme, None, self.cursor.line));
    }
}

#[derive(Debug)]
pub struct Cursor<'a> {
    src: &'a str,
    iter: std::str::CharIndices<'a>,
    position: usize,
    line: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src,
            iter: src.char_indices(),
            position: 0,
            line: 1,
        }
    }

    pub fn advance(&mut self) -> Option<char> {
        let (current, c) = self.iter.next()?;
        self.position = current + c.len_utf8();

        if matches!(c, '\n') {
            self.line += 1;
        }

        Some(c)
    }

    pub const fn is_at_end(&self) -> bool {
        self.position >= self.src.len()
    }

    pub fn peek_next(&self) -> Option<char> {
        let mut iter = self.iter.clone();
        iter.next();
        iter.next().map(|(_, c)| c)
    }

    pub fn peek(&self) -> Option<char> {
        self.iter.clone().next().map(|(_, c)| c)
    }

    pub fn slice_from(&self, start: usize) -> &'a str {
        &self.src[start..self.position]
    }
}
