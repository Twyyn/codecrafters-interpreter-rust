pub enum Expression {
    Literal(Literal),
}

pub enum Literal {
    Number,
    String,
    True,
    False,
    Nil,
}

pub enum Binary {
    Not,
    Negate,
}
