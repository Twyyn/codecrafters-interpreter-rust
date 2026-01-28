use crate::{
    ast::{Expr, LiteralValue},
    token::TokenType,
};

pub struct Interpreter;

impl Interpreter {
    pub fn evaluate(expr: Expr) -> Result<LiteralValue, RuntimeError> {
        match expr {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = Interpreter::evaluate(*left)?;
                let right = Interpreter::evaluate(*right)?;

                match operator.token_type {
                    TokenType::PLUS => match (left, right) {
                        (LiteralValue::Number(l), LiteralValue::Number(r)) => {
                            Ok(LiteralValue::Number(l + r))
                        }
                        (LiteralValue::String(l), LiteralValue::String(r)) => {
                            Ok(LiteralValue::String(format!("{}{}", l, r)))
                        }
                        _ => Err(RuntimeError {
                            line: operator.line,
                            message: "Operands must be two numbers or two strings.".to_string(),
                        }),
                    },

                    _ => unimplemented!(),
                }
            }
            Expr::Literal(value) => Ok(value),
            Expr::Grouping(expr) => Interpreter::evaluate(*expr),
            Expr::Unary { operator, right } => {
                let right = Interpreter::evaluate(*right)?;

                match operator.token_type {
                    TokenType::MINUS => match right {
                        LiteralValue::Number(n) => Ok(LiteralValue::Number(-n)),

                        _ => Err(RuntimeError {
                            line: operator.line,
                            message: "Operand must be a number.".to_string(),
                        }),
                    },
                    _ => Err(RuntimeError {
                        line: operator.line,
                        message: format!("Unsupported unary operator: {:?}", operator.token_type),
                    }),
                }
            }
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
