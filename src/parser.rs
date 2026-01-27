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

    // === Core Parsing Methods ===

    pub fn parse(&mut self) -> Result<Expr, String> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, String> {
        match self.peek().token_type {
            TokenType::TRUE => {
                self.advance();
                Ok(Expr::Literal {
                    value: LiteralValue::Boolean(true),
                })
            }
            TokenType::FALSE => {
                self.advance();
                Ok(Expr::Literal {
                    value: LiteralValue::Boolean(false),
                })
            }
            TokenType::NIL => {
                self.advance();
                Ok(Expr::Literal {
                    value: LiteralValue::Nil,
                })
            }
            TokenType::NUMBER => {
                let token = self.advance();
                let s = token
                    .literal
                    .as_deref()
                    .ok_or_else(|| "Expected number literal".to_string())?;

                let number = s
                    .parse::<f64>()
                    .map_err(|e| format!("Invalid number literal '{s}': {e}"))?;

                Ok(Expr::Literal {
                    value: LiteralValue::Number(number),
                })
            }
            TokenType::STRING => {
                let token = self.advance();
                let s = token
                    .literal
                    .as_deref()
                    .ok_or_else(|| "Expected string literal".to_string())?
                    .to_string();
                Ok(Expr::Literal {
                    value: LiteralValue::String(s),
                })
            }

            TokenType::LEFT_PAREN => {
                self.advance();
                let expr = self.expression()?;
                self.consume(TokenType::RIGHT_PAREN, "Expect ')' after expression.")?;

                Ok(Expr::Grouping {
                    expression: Box::new(expr),
                })
            }
            _ => Err(self.error(self.peek().clone(), "Expected expression")),
        }
    }

    // === Parser Navigation ===

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
        self.current >= self.tokens.len() || self.tokens[self.current].token_type == TokenType::EOF
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    // === Error ===

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, String> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        Err(self.error(self.peek().clone(), message))
    }

    fn error(&mut self, token: Token, message: &str) -> String {
        if token.token_type == TokenType::EOF {
            self.report(token.line, " at end", message);
        } else {
            self.report(token.line, &format!(" at '{}'", token.lexeme), message);
        }
        message.to_string()
    }

    fn report(&mut self, line: usize, where_msg: &str, message: &str) {
        eprintln!("[line {}] Error:{} {}", line, where_msg, message);
    }
}
