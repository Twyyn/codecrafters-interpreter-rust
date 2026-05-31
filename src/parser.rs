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

    pub fn expression(&mut self) -> Result<Expr<'a>, ParseError> {
        self.primary()
    }

    // fn unary(&mut self) -> Result<Expr<'a>, ParseError> {
    //     let expr = self.primary()?;
    //     todo!()
    // }

    fn primary(&mut self) -> Result<Expr<'a>, ParseError> {
        if self.cursor.match_token(TokenKind::True) {
            return Ok(Expr::Literal(Literal::Boolean(true)));
        }

        if self.cursor.match_token(TokenKind::False) {
            return Ok(Expr::Literal(Literal::Boolean(false)));
        }

        if self.cursor.match_token(TokenKind::Nil) {
            return Ok(Expr::Literal(Literal::Nil));
        }

        if self.cursor.match_token(TokenKind::Number)
            && let Some(crate::token::Literal::Number(number)) = self
                .cursor
                .previous()
                .and_then(|token| token.literal.as_ref())
        {
            return Ok(Expr::Literal(Literal::Number(*number)));
        }

        if self.cursor.match_token(TokenKind::String)
            && let Some(token) = self.cursor.previous()
        {
            return Ok(Expr::Literal(Literal::String(token.lexeme)));
        }

        if self.cursor.match_token(TokenKind::LeftParen) {
            let expr = self.expression()?;
            self.cursor.consume(TokenKind::RightParen)?;
            return Ok(Expr::Grouping(Box::new(expr)));
        }

        Err(ParseError::UnexpectedExpr)
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

    #[allow(clippy::unwrap_used)]
    pub fn consume(&mut self, kind: TokenKind) -> Result<&Token<'a>, ParseError> {
        if self.check_token(&kind) {
            return Ok(self.advance().unwrap());
        }

        Err(ParseError::UnmatchedParentheses {
            line: self.peek().map_or(0, |token| token.line),
        })
    }

    pub fn match_token(&mut self, kind: TokenKind) -> bool {
        self.match_tokens(&[kind])
    }

    pub fn match_tokens(&mut self, kinds: &[TokenKind]) -> bool {
        for token in kinds {
            if self.check_token(token) {
                self.advance();
                return true;
            }
        }
        false
    }

    pub fn check_token(&self, kind: &TokenKind) -> bool {
        self.peek().is_some_and(|token| token.kind == *kind)
    }

    pub fn advance(&mut self) -> Option<&Token<'a>> {
        let token = self.tokens.get(self.position);

        if token.is_some() {
            self.position += 1;
        }

        token
    }

    pub fn previous(&self) -> Option<&Token<'a>> {
        self.tokens.get(self.position - 1)
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
    #[error("Error: Expected expression")]
    UnexpectedExpr,
    #[error("[line {line}] Error: Unmatched parentheses.")]
    UnmatchedParentheses { line: usize },
}
