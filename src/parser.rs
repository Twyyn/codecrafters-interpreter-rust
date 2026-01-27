use crate::ast::{Expr, LiteralValue};
use crate::token::{Token, TokenType};

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr, String> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, String> {
        if self.match_types(&[TokenType::TRUE]) {
            Ok(Expr::Literal {
                value: (LiteralValue::Boolean(true)),
            })
        } else if self.match_types(&[TokenType::FALSE]) {
            Ok(Expr::Literal {
                value: (LiteralValue::Boolean(false)),
            })
        } else {
            Ok(Expr::Literal {
                value: (LiteralValue::Nil),
            })
        }
    }

    fn match_types(&mut self, token_types: &[TokenType]) -> bool {
        for &t in token_types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len() || self.tokens[self.current].token_type == TokenType::EOF
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }
}
