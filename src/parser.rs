use crate::{
    grammar::{Expr, Literal},
    token::{Token, TokenKind},
};
use thiserror::Error;

pub struct Parser<'a> {
    cursor: ParserCursor<'a>,
}

impl<'a> Parser<'a> {
    pub const fn new(tokens: &'a [Token<'a>]) -> Self {
        Self {
            cursor: ParserCursor::new(tokens),
        }
    }

    pub fn parse(&mut self) -> Vec<Expr> {
        let mut expressions = Vec::new();
        while !self.cursor.is_at_end() {
            expressions.push(self.primary());
        }

        expressions
    }

    fn primary(&mut self) -> Expr {
        if self.cursor.match_token_kinds(&[TokenKind::True]) {
            return Expr::Literal(Literal::Boolean(true));
        }

        if self.cursor.match_token_kinds(&[TokenKind::False]) {
            return Expr::Literal(Literal::Boolean(false));
        }

        if self.cursor.match_token_kinds(&[TokenKind::Nil]) {
            return Expr::Literal(Literal::Nil);
        }

        panic!("expected expression");
    }
}

pub struct ParserCursor<'a> {
    tokens: &'a [Token<'a>],
    position: usize,
}

impl<'a> ParserCursor<'a> {
    pub const fn new(tokens: &'a [Token<'a>]) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    pub fn match_token_kinds(&mut self, tokens: &[TokenKind]) -> bool {
        for token in tokens {
            if self.check_token_kind(token) {
                self.advance();
                return true;
            }
        }
        false
    }

    pub fn check_token_kind(&self, token_kind: &TokenKind) -> bool {
        self.peek().is_some_and(|t| t.kind == *token_kind)
    }

    pub fn advance(&mut self) -> Option<&Token<'a>> {
        let token = self.tokens.get(self.position);

        if token.is_some() {
            self.position += 1;
        }

        token
    }

    pub fn previous(&self) -> Option<&Token<'a>> {
        if self.position == 0 {
            None
        } else {
            self.tokens.get(self.position - 1)
        }
    }

    pub fn is_at_end(&self) -> bool {
        matches!(
            self.peek(),
            Some(Token {
                kind: TokenKind::EOF,
                ..
            })
        )
    }

    pub fn peek(&self) -> Option<&Token<'a>> {
        self.tokens.get(self.position)
    }
}

#[derive(Debug, Error)]
pub enum ParseError {
    // #[error("[line {line}] Error: Unexpected character: {c}")]
    // UnexpectedChar { line: usize, c: char },
}
