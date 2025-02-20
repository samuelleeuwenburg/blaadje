// https://norvig.com/lispy.html
#![allow(dead_code)]

mod env;
mod eval;
mod parse;

pub use env::Environment;
pub use eval::{eval, run_program};
pub use parse::parse;

#[derive(Debug, Clone, PartialEq)]
pub enum Blad {
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
    Let,
    Add,
    List,
    Lambda,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BladError {
    UnexpectedToken(String),
    ExpectedSymbol,
    InvalidToken(String),
    UnsupportedNumericType(String),
    UndefinedSymbol,
    UndefinedOperator,
    ExpectedProcedure,
    ExpectedList,
    IncorrectLambdaSyntax,
    WrongNumberOfArguments,
    AttemptToRedefineVariable,
}
