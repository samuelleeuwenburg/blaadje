// https://norvig.com/lispy.html
#![allow(dead_code)]

mod env;
mod eval;
mod parse;
mod prelude;

pub use env::Environment;
pub use eval::{eval, run_program};
pub use parse::parse;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum Blad {
    Unit,
    List(Vec<Blad>),
    Literal(Literal),
    Symbol(String),
    Quote(Box<Blad>),
    Lambda(Rc<RefCell<Environment>>, Vec<String>, Box<Blad>),
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
    ExpectedBoolean,
    ExpectedF32,
    ExpectedList,
    ExpectedNumber,
    ExpectedProcedure,
    ExpectedSymbol,
    ExpectedUsize,
    ExpectedSameTypes,
    IncorrectLambdaSyntax,
    InvalidToken(String),
    UndefinedOperator,
    UndefinedSymbol,
    UnexpectedToken(String),
    UnsupportedNumericType(String),
    WrongNumberOfArguments,
}
