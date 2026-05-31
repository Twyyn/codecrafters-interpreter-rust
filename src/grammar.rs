use std::fmt;

#[derive(Debug, Clone)]

pub enum Expr<'a> {
    Literal(Literal<'a>),
}

impl fmt::Display for Expr<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(literal) => write!(f, "{literal}"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Literal<'a> {
    Number(f64),
    String(&'a str),
    Boolean(bool),
    Nil,
}

impl fmt::Display for Literal<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(number) => write!(f, "{number}"),
            Self::String(string) => write!(f, "{string}"),
            Self::Boolean(bool) => write!(f, "{bool}"),
            Self::Nil => write!(f, "nil"),
        }
    }
}
