use std::fmt;

#[derive(Debug, Clone)]

pub enum Expr<'a> {
    Literal(Literal<'a>),
    Grouping(Box<Self>),
    Binary {
        left_operand: Box<Self>,
        operator: Operator,
        right_operand: Box<Self>,
    },
    Unary {
        operator: Operator,
        operand: Box<Self>,
    },
}

#[derive(Debug, Clone)]
pub enum Literal<'a> {
    Number(f64),
    String(&'a str),
    Boolean(bool),
    Nil,
}

#[derive(Debug, Clone)]

pub enum Operator {
    Add,
    Subtract,
    Divide,
    GreaterThan,
    LessThan,
    GreaterThanEqual,
    LessThanEqual,
    EqualEqual,
    NotEqual,
    And,
    Or,
}

impl fmt::Display for Expr<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(literal) => write!(f, "{literal}"),
            Self::Grouping(expr) => write!(f, "(group {expr})"),
            Self::Binary {
                left_operand,
                operator,
                right_operand,
            } => write!(f, "{operator} {left_operand} {right_operand}"),
            Self::Unary { operator, operand } => write!(f, "{operator} {operand}"),
        }
    }
}

impl fmt::Display for Literal<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(number) => {
                if number.fract() == 0.0 {
                    write!(f, "{number:.1}")
                } else {
                    write!(f, "{number}")
                }
            }
            Self::String(string) => write!(f, "{string}"),
            Self::Boolean(bool) => write!(f, "{bool}"),
            Self::Nil => write!(f, "nil"),
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Add => "+",
            Self::Subtract => "-",
            Self::Divide => "/",
            Self::GreaterThan => ">",
            Self::LessThan => "<",
            Self::GreaterThanEqual => ">=",
            Self::LessThanEqual => "<=",
            Self::EqualEqual => "==",
            Self::NotEqual => "!=",
            Self::And => "&",
            Self::Or => "|",
        };
        write!(f, "{s}")
    }
}
