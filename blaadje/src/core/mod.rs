mod env;
mod eval;
mod operators;
mod parse;

use super::audio::{Message, Screech};
pub use env::Environment;
pub use eval::{args, args_min, eval};
pub use parse::parse;

#[derive(Debug, Clone)]
pub enum Blad {
    Atom(String),
    Keyword(Keyword),
    Lambda(Environment, Vec<String>, Box<Blad>),
    List(Vec<Blad>),
    Literal(Literal),
    Macro(Vec<String>, Box<Blad>),
    Quote(Box<Blad>),
    Screech(Screech),
    Symbol(String),
    Unit,
}

impl PartialEq for Blad {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Blad::Atom(a), Blad::Atom(b)) => a == b,
            (Blad::Keyword(a), Blad::Keyword(b)) => a == b,
            (Blad::List(a), Blad::List(b)) => a == b,
            (Blad::Literal(a), Blad::Literal(b)) => a == b,
            (Blad::Quote(a), Blad::Quote(b)) => a == b,
            (Blad::Symbol(a), Blad::Symbol(b)) => a == b,
            (Blad::Unit, Blad::Unit) => true,
            _ => false,
        }
    }
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
    ExpectedAtom(Blad),
    ExpectedF32(Blad),
    ExpectedList(Blad),
    ExpectedNumber(Blad),
    ExpectedProcedure(Blad),
    ExpectedSameTypes(Blad, Blad),
    ExpectedScreechModule(Blad),
    ExpectedScreechSignal(Blad),
    ExpectedSymbol(Blad),
    ExpectedUsize(Blad),
    IncorrectArguments(Vec<BladError>),
    IncorrectLambdaSyntax(Blad),
    IncorrectMacroSyntax(Blad),
    IncorrectNumberOfArguments(usize, usize),
    InvalidToken(String),
    ParseError(usize),
    UndefinedOperator(String),
    UndefinedSymbol(String),
    UnexpectedMessage(Message),
    UnexpectedToken(String),
    UnsupportedNumericType(String),
}
