mod env;
mod eval;
mod operators;
mod parse;

pub use env::Environment;
pub use eval::eval;
pub use parse::parse;

#[derive(Debug, Clone, PartialEq)]
pub enum Blad {
    Keyword(Keyword),
    Lambda(Environment, Vec<String>, Box<Blad>),
    List(Vec<Blad>),
    Literal(Literal),
    Macro(Vec<String>, Box<Blad>),
    Quote(Box<Blad>),
    Symbol(String),
    Unit,
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
    Append,
    Cons,
    Do,
    Equal,
    GreaterThan,
    Head,
    If,
    Lambda,
    LessThan,
    Let,
    List,
    Macro,
    Subtract,
    Tail,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BladError {
    AttemptToRedefineVariable(String),
    ExpectedF32(Blad),
    ExpectedList(Blad),
    ExpectedNumber(Blad),
    ExpectedProcedure(Blad),
    ExpectedSameTypes(Blad, Blad),
    ExpectedSymbol(Blad),
    ExpectedUsize(Blad),
    IncorrectLambdaSyntax(Blad),
    IncorrectMacroSyntax(Blad),
    InvalidToken(String),
    ParseError(usize),
    UndefinedSymbol(String),
    UnexpectedToken(String),
    UnsupportedNumericType(String),
    WrongNumberOfArguments(usize, usize),
}
