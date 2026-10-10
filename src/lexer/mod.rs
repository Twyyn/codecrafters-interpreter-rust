mod token;

use std::{iter::Peekable, num::ParseFloatError, str::Chars};

use self::token::{ErrorToken, LiteralValue, Token, TokenKind};
use thiserror::Error;

type LexResult<'a> = Result<Token<'a>, ErrorToken<LexError>>;

#[derive(Debug)]
pub struct Lexer<'a> {
    src: &'a str,
    chars: Peekable<Chars<'a>>,
    cursor: usize,
    offset: usize,
    line_offset: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            src: input,
            chars: input.chars().peekable(),
            cursor: 0,
            offset: 0,
            line_offset: 1,
        }
    }

    fn next_token(&mut self) -> LexResult<'a> {
        self.skip_trivia();

        self.offset = self.cursor;

        let Some(ch) = self.bump() else {
            return Ok(Token::new(TokenKind::Eof, "", None));
        };

        let kind = match ch {
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            '{' => TokenKind::LeftBrace,
            '}' => TokenKind::RightBrace,
            ',' => TokenKind::Comma,
            '.' => TokenKind::Dot,
            ';' => TokenKind::Semicolon,
            '-' => TokenKind::Minus,
            '+' => TokenKind::Plus,
            '/' => TokenKind::Slash,
            '*' => TokenKind::Star,
            '!' => {
                if self.consume_next('=') {
                    TokenKind::BangEqual
                } else {
                    TokenKind::Bang
                }
            }
            '=' => {
                if self.consume_next('=') {
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
                }
            }

            '>' => {
                if self.consume_next('=') {
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                }
            }

            '<' => {
                if self.consume_next('=') {
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                }
            }

            '"' => return self.string(),
            '0'..='9' => return self.number(),
            'a'..='z' | 'A'..='Z' | '_' => return Ok(self.identifier()),

            _ => {
                let error = LexError::UnexpectedChar(ch);
                return Err(ErrorToken::new(error, self.line_offset));
            }
        };

        Ok(Token::new(kind, self.lexeme(), None))
    }

    fn identifier(&mut self) -> Token<'a> {
        self.consume_while(|ch| ch.is_ascii_alphanumeric() || ch == '_');

        let lexeme = self.lexeme();

        let kind = match lexeme {
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "and" => TokenKind::And,
            "or" => TokenKind::Or,
            "nil" => TokenKind::Nil,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "for" => TokenKind::For,
            "while" => TokenKind::While,
            "return" => TokenKind::Return,
            "class" => TokenKind::Class,
            "fun" => TokenKind::Fun,
            "super" => TokenKind::Super,
            "this" => TokenKind::This,
            "var" => TokenKind::Var,
            "print" => TokenKind::Print,
            _ => TokenKind::Identifier,
        };

        let literal = if kind.is_identifier() {
            Some(LiteralValue::String(lexeme))
        } else {
            None
        };

        Token::new(kind, lexeme, literal)
    }

    fn string(&mut self) -> LexResult<'a> {
        let start_line = self.line_offset;

        while let Some(ch) = self.bump() {
            match ch {
                '"' => {
                    let lexeme = self.lexeme();
                    let literal = &self.src[self.offset + 1..self.cursor - 1];

                    return Ok(Token::new(
                        TokenKind::String,
                        lexeme,
                        Some(LiteralValue::String(literal)),
                    ));
                }
                '\n' => break,
                _ => {}
            }
        }

        Err(ErrorToken::new(LexError::UnterminatedString, start_line))
    }

    fn number(&mut self) -> LexResult<'a> {
        self.consume_while(|ch| ch.is_ascii_digit());

        if self.peek() == Some('.') && self.peek_next().is_some_and(|ch| ch.is_ascii_digit()) {
            self.bump();
            self.consume_while(|ch| ch.is_ascii_digit());
        }

        let lexeme = self.lexeme();
        let value = lexeme.parse::<f64>().map_err(|source| {
            let error = LexError::InvalidNumber {
                text: lexeme.into(),
                source,
            };

            ErrorToken::new(error, self.line_offset)
        })?;

        Ok(Token::new(
            TokenKind::Number,
            lexeme,
            Some(LiteralValue::Number(value)),
        ))
    }

    fn skip_trivia(&mut self) {
        loop {
            self.consume_while(|ch| ch.is_ascii_whitespace());

            if self.peek() != Some('/') || self.peek_next() != Some('/') {
                break;
            }

            self.bump();
            self.bump();

            self.consume_while(|ch| ch != '\n');
        }
    }

    fn consume_while(&mut self, predicate: impl Fn(char) -> bool) {
        while self.peek().is_some_and(&predicate) {
            self.bump();
        }
    }

    fn consume_next(&mut self, expected: char) -> bool {
        if self.peek() != Some(expected) {
            return false;
        }

        self.bump();
        true
    }

    fn bump(&mut self) -> Option<char> {
        let ch = self.chars.next()?;

        self.cursor += ch.len_utf8();

        if ch == '\n' {
            self.line_offset += 1;
        }

        Some(ch)
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    fn peek_next(&self) -> Option<char> {
        self.chars.clone().nth(1)
    }

    fn lexeme(&self) -> &'a str {
        &self.src[self.offset..self.cursor]
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = LexResult<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Ok(token) if token.kind.is_eof() => None,
            result => Some(result),
        }
    }
}

#[derive(Debug, Error)]
pub enum LexError {
    #[error("Unexpected character: {0}")]
    UnexpectedChar(char),

    #[error("Unterminated string.")]
    UnterminatedString,

    #[error("Invalid number '{text}': {source}")]
    InvalidNumber {
        text: String,
        #[source]
        source: ParseFloatError,
    },
}
