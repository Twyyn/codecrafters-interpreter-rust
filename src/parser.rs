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

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        let token = self.peek();

        let expr = match token.token_type {
            TokenType::TRUE => Expr::Literal(LiteralValue::Boolean(true)),
            TokenType::FALSE => Expr::Literal(LiteralValue::Boolean(false)),
            TokenType::NIL => Expr::Literal(LiteralValue::Nil),

            TokenType::NUMBER => {
                let n = token
                    .literal
                    .as_deref()
                    .ok_or_else(|| self.error("Expected number literal"))?
                    .parse::<f64>()
                    .map_err(|_| self.error("Invalid number literal"))?;

                Expr::Literal(LiteralValue::Number(n))
            }

            TokenType::STRING => {
                let s = token
                    .literal
                    .clone()
                    .ok_or_else(|| self.error("Expected string literal"))?;
                Expr::Literal(LiteralValue::String(s))
            }

            TokenType::LEFT_PAREN => {
                self.advance();
                let inner = self.expression()?;
                self.consume(TokenType::RIGHT_PAREN, "Expect ')' after expression")?;
                return Ok(Expr::Grouping(Box::new(inner)));
            }

            _ => return Err(self.error("Expected expression")),
        };

        self.advance();
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_any(&[TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous().clone();
            let right = self.unary()?;

            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }

        self.primary()
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;
        while self.match_any(&[TokenType::STAR, TokenType::SLASH]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;
        while self.match_any(&[TokenType::PLUS, TokenType::MINUS]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;
        while self.match_any(&[
            TokenType::GREATER,
            TokenType::GREATER_EQUAL,
            TokenType::LESS,
            TokenType::LESS_EQUAL,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;
        while self.match_any(&[TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }
    // === Navigation ===

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        matches!(
            self.tokens.get(self.current),
            None | Some(Token {
                token_type: TokenType::EOF,
                ..
            })
        )
    }

    fn check(&self, token_type: TokenType) -> bool {
        !self.is_at_end() && self.peek().token_type == token_type
    }

    fn match_any(&mut self, token_types: &[TokenType]) -> bool {
        if token_types.iter().any(|&t| self.check(t)) {
            self.advance();
            return true;
        }
        false
    }
    // === Errors ===

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, ParseError> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(self.error(message))
        }
    }

    fn error(&self, message: &str) -> ParseError {
        let token = self.peek();
        ParseError {
            line: token.line,
            location: if token.token_type == TokenType::EOF {
                "at end".to_string()
            } else {
                format!("at '{}'", token.lexeme)
            },
            message: message.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct ParseError {
    pub line: usize,
    pub location: String,
    pub message: String,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[line {}] Error {}: {}",
            self.line, self.location, self.message
        )
    }
}

impl std::error::Error for ParseError {}
