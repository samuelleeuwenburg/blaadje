mod env;
mod eval;
mod operators;
mod parse;

pub use env::Environment;
pub use eval::eval;
pub use parse::parse;

#[derive(Debug, Clone, PartialEq)]
pub enum Blad {
    Unit,
    List(Vec<Blad>),
    Literal(Literal),
    Symbol(String),
    Quote(Box<Blad>),
    Lambda(Environment, Vec<String>, Box<Blad>),
    Keyword(Keyword),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Usize(usize),
    F32(f32),
    String(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Add,
    Do,
    Head,
    If,
    Lambda,
    Let,
    List,
    Subtract,
    Tail,
    Equal,
    GreaterThan,
    LessThan,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BladError {
    AttemptToRedefineVariable(String),
    ExpectedF32(Blad),
    ExpectedList(Blad),
    ExpectedNumber(Blad),
    ExpectedProcedure(Blad),
    ExpectedSymbol(Blad),
    ExpectedUsize(Blad),
    ExpectedSameTypes(Blad, Blad),
    IncorrectLambdaSyntax(Blad),
    InvalidToken(String),
    UndefinedSymbol(String),
    UnexpectedToken(String),
    UnsupportedNumericType(String),
    WrongNumberOfArguments(usize, usize),
}
