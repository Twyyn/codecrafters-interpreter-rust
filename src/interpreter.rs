use crate::{
    ast::{Expr, LiteralValue, Statement},
    environment::Environment,
    token::{Token, TokenType},
};
#[derive(Debug, Default)]
pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }
    pub fn evaluate(&mut self, expr: Expr) -> Result<LiteralValue, RuntimeError> {
        match expr {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.evaluate(*left)?;
                let right = self.evaluate(*right)?;
                match operator.token_type {
                    // Arithmetic
                    TokenType::PLUS => match (&left, &right) {
                        (LiteralValue::Number(left), LiteralValue::Number(right)) => {
                            Ok(LiteralValue::Number(left + right))
                        }
                        (LiteralValue::String(left), LiteralValue::String(right)) => {
                            Ok(LiteralValue::String(format!("{left}{right}")))
                        }
                        _ => Err(Self::error(
                            &operator,
                            "Operands must be two numbers or two strings.",
                        )),
                    },
                    TokenType::MINUS => {
                        let (left, right) = Self::expect_numbers(&operator, &left, &right)?;
                        Ok(LiteralValue::Number(left - right))
                    }
                    TokenType::STAR => {
                        let (left, right) = Self::expect_numbers(&operator, &left, &right)?;
                        Ok(LiteralValue::Number(left * right))
                    }
                    TokenType::SLASH => {
                        let (left, right) = Self::expect_numbers(&operator, &left, &right)?;
                        Ok(LiteralValue::Number(left / right))
                    }

                    // Comparison
                    TokenType::GREATER => {
                        let (left, right) = Self::expect_numbers(&operator, &left, &right)?;
                        Ok(LiteralValue::Boolean(left > right))
                    }
                    TokenType::GREATER_EQUAL => {
                        let (left, right) = Self::expect_numbers(&operator, &left, &right)?;
                        Ok(LiteralValue::Boolean(left >= right))
                    }
                    TokenType::LESS => {
                        let (left, right) = Self::expect_numbers(&operator, &left, &right)?;
                        Ok(LiteralValue::Boolean(left < right))
                    }
                    TokenType::LESS_EQUAL => {
                        let (left, right) = Self::expect_numbers(&operator, &left, &right)?;
                        Ok(LiteralValue::Boolean(left <= right))
                    }

                    // Equality
                    TokenType::EQUAL_EQUAL => {
                        Ok(LiteralValue::Boolean(Self::is_equal(&left, &right)))
                    }
                    TokenType::BANG_EQUAL => {
                        Ok(LiteralValue::Boolean(!Self::is_equal(&left, &right)))
                    }
                    _ => unreachable!(),
                }
            }
            Expr::Literal(value) => Ok(value),
            Expr::Grouping(inner) => self.evaluate(*inner),
            Expr::Unary { operator, right } => {
                let right = self.evaluate(*right)?;
                match operator.token_type {
                    TokenType::MINUS => {
                        let n = Self::expect_number(&operator, &right)?;
                        Ok(LiteralValue::Number(-n))
                    }
                    TokenType::BANG => Ok(LiteralValue::Boolean(!Self::is_truthy(&right))),
                    _ => unreachable!(),
                }
            }
            Expr::Variable(name) => {
                self.environment
                    .get(&name.lexeme)
                    .cloned()
                    .ok_or_else(|| RuntimeError {
                        line: name.line,
                        message: format!("Undefined variable '{}'", name.lexeme),
                    })
            }
            Expr::Assignment { name, value } => {
                let value = self.evaluate(*value)?;
                self.environment.assign(&name.lexeme, value.clone())?;
                Ok(value)
            }
        }
    }

    pub fn run(&mut self, statement: Statement) -> Result<(), RuntimeError> {
        match statement {
            Statement::Print(expr) => {
                let value = self.evaluate(expr)?;
                println!("{}", value.as_string());
                Ok(())
            }
            Statement::Expression(expr) => {
                self.evaluate(expr)?;
                Ok(())
            }
            Statement::Var { name, initializer } => {
                let value = if let Some(initializer) = initializer {
                    self.evaluate(initializer)?
                } else {
                    LiteralValue::Nil
                };

                self.environment.define(name.lexeme.clone(), value);
                Ok(())
            }
            Statement::Block(statements) => {
                let previous = std::mem::take(&mut self.environment);
                self.environment = Environment::with_enclosing(previous);

                let result = statements.into_iter().try_for_each(|s| self.run(s));

                let environment = std::mem::take(&mut self.environment);
                self.environment = environment.into_enclosing().unwrap();
                result
            }
        }
    }

    fn is_truthy(value: &LiteralValue) -> bool {
        !matches!(value, LiteralValue::Nil | LiteralValue::Boolean(false))
    }

    fn is_equal(left: &LiteralValue, right: &LiteralValue) -> bool {
        match (left, right) {
            (LiteralValue::Number(left), LiteralValue::Number(right)) => left == right,
            (LiteralValue::Nil, LiteralValue::Nil) => true,
            (LiteralValue::Boolean(left), LiteralValue::Boolean(right)) => left == right,
            (LiteralValue::String(left), LiteralValue::String(right)) => left == right,
            _ => false,
        }
    }

    fn expect_number(operator: &Token, value: &LiteralValue) -> Result<f64, RuntimeError> {
        match value {
            LiteralValue::Number(n) => Ok(*n),
            _ => Err(Self::error(operator, "Operand must be a number.")),
        }
    }

    fn expect_numbers(
        operator: &Token,
        left: &LiteralValue,
        right: &LiteralValue,
    ) -> Result<(f64, f64), RuntimeError> {
        match (left, right) {
            (LiteralValue::Number(l), LiteralValue::Number(r)) => Ok((*l, *r)),
            _ => Err(Self::error(operator, "Operands must be numbers.")),
        }
    }

    fn error(operator: &Token, message: &str) -> RuntimeError {
        RuntimeError {
            line: operator.line,
            message: message.to_string(),
        }
    }
}

pub struct RuntimeError {
    pub line: usize,
    pub message: String,
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[line {}] Error: {}", self.line, self.message)
    }
}
