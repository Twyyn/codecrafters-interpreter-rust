use crate::ast::{Expr, LiteralValue, Statement};
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

    pub fn parse_statements(&mut self) -> Result<Vec<Statement>, ParseError> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            statements.push(self.statement()?);
        }

        Ok(statements)
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

            TokenType::IDENTIFIER => {
                let name = self.peek().clone();
                self.advance();
                return Ok(Expr::Variable(name));
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

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.logical_or()?;

        if self.match_any(&[TokenType::EQUAL]) {
            let value = self.assignment()?;

            if let Expr::Variable(name) = expr {
                return Ok(Expr::Assignment {
                    name,
                    value: Box::new(value),
                });
            }
            return Err(self.error("Invalid assignment target."));
        }

        if self.match_any(&[TokenType::OR]) {
            self.logical_or()?;
        }

        Ok(expr)
    }
    fn statement(&mut self) -> Result<Statement, ParseError> {
        if self.match_any(&[TokenType::PRINT]) {
            return self.print_statement();
        }
        if self.match_any(&[TokenType::VAR]) {
            return self.var_statement();
        }
        if self.match_any(&[TokenType::LEFT_BRACE]) {
            return Ok(Statement::Block(self.block()?));
        }
        if self.match_any(&[TokenType::IF]) {
            return self.if_statement();
        }
        self.expression_statement()
    }

    fn var_statement(&mut self) -> Result<Statement, ParseError> {
        let name = self
            .consume(TokenType::IDENTIFIER, "Expect variable name.")?
            .clone();
        let initializer = if self.check(TokenType::EQUAL) {
            self.advance();
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(
            TokenType::SEMICOLON,
            "Expect ';' after variable declaration.",
        )?;
        Ok(Statement::Var { name, initializer })
    }

    fn print_statement(&mut self) -> Result<Statement, ParseError> {
        let value = self.expression()?;
        self.consume(TokenType::SEMICOLON, "Expect ';' after value.")?;
        Ok(Statement::Print(value))
    }

    fn expression_statement(&mut self) -> Result<Statement, ParseError> {
        let expr = self.expression()?;
        self.consume(TokenType::SEMICOLON, "Expect ';' after expression.")?;
        Ok(Statement::Expression(expr))
    }

    fn block(&mut self) -> Result<Vec<Statement>, ParseError> {
        let mut statements: Vec<Statement> = Vec::new();
        while !self.check(TokenType::RIGHT_BRACE) && !self.is_at_end() {
            statements.push(self.statement()?);
        }
        self.consume(TokenType::RIGHT_BRACE, "Expect '}' after block.")?;
        Ok(statements)
    }

    fn if_statement(&mut self) -> Result<Statement, ParseError> {
        self.consume(TokenType::LEFT_PAREN, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RIGHT_PAREN, "Expect ')' after if condition.")?;

        let then_branch = Box::new(self.statement()?);

        let else_branch = if self.match_any(&[TokenType::ELSE]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(Statement::If {
            condition,
            then_branch,
            else_branch,
        })
    }
    fn logical_or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.logical_and()?;

        while self.match_any(&[TokenType::OR]) {
            let operator = self.previous().clone();
            let right = self.logical_and()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn logical_and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.equality()?;

        while self.match_any(&[TokenType::AND]) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            expr = Expr::Logical {
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
