use crate::{
    errors::InterpreterError,
    token::{Token, TokenKind},
};

#[derive(Debug)]
pub struct Lexer<'a> {
    cursor: Cursor<'a>,
    tokens: Vec<Token<'a>>,
    start: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            cursor: Cursor::new(src),
            tokens: Vec::new(),
            start: 0,
        }
    }

    pub fn scan_tokens(mut self) -> Result<Vec<Token<'a>>, InterpreterError> {
        loop {
            self.start = self.cursor.position();

            let Some(c) = self.cursor.advance() else {
                break;
            };

            match c {
                '(' => self.add_token(TokenKind::LeftParen),
                ')' => self.add_token(TokenKind::RightParen),
                '{' => self.add_token(TokenKind::LeftBrace),
                '}' => self.add_token(TokenKind::RightBrace),

                ',' => self.add_token(TokenKind::Comma),
                '.' => self.add_token(TokenKind::Dot),
                '-' => self.add_token(TokenKind::Minus),
                '+' => self.add_token(TokenKind::Plus),
                ';' => self.add_token(TokenKind::Semicolon),
                '/' => self.add_token(TokenKind::Slash),
                '*' => self.add_token(TokenKind::Star),

                // c if c.is_ascii_digit() => {
                //     while let
                // }
                ' ' | '\r' | '\t' | '\n' => {}
                _ => {
                    return Err(InterpreterError::Lex {
                        line: self.cursor.line,
                        message: format!("Unexpected character: {c}"),
                    });
                }
            }
        }

        self.tokens
            .push(Token::new(TokenKind::EOF, "", None, self.cursor.line));
        Ok(self.tokens)
    }

    fn add_token(&mut self, kind: TokenKind) {
        self.tokens.push(Token::new(
            kind,
            self.cursor.slice_from(self.start),
            None,
            self.cursor.line,
        ));
    }
}

#[derive(Debug)]
pub struct Cursor<'a> {
    src: &'a str,
    iter: std::iter::Peekable<std::str::CharIndices<'a>>,
    position: usize,
    line: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src,
            iter: src.char_indices().peekable(),
            position: 0,
            line: 1,
        }
    }

    pub fn advance(&mut self) -> Option<char> {
        let (offset, c) = self.iter.next()?;
        self.position = offset + c.len_utf8();

        if matches!(c, '\n') {
            self.line += 1;
        }

        Some(c)
    }

    pub fn peek(&mut self) -> Option<char> {
        self.iter.peek().map(|&(_, c)| c)
    }

    pub fn peek_next(&self) -> Option<char> {
        let mut ahead = self.src.char_indices().skip(self.position);
        ahead.next();
        ahead.next().map(|(_, c)| c)
    }

    pub fn slice_from(&self, start: usize) -> &'a str {
        &self.src[start..self.position]
    }

    pub const fn position(&self) -> usize {
        self.position
    }
}
