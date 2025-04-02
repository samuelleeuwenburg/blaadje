mod channel;
mod env;
mod error;
mod eval;
pub mod notes;
mod operators;
mod parse;

pub use channel::Channel;
pub use env::Environment;
pub use error::Error;
pub use eval::{args, args_min, eval};
use notes::atom_to_pitch;
pub use parse::parse;
use screech::Signal;

#[derive(Debug, Clone)]
pub enum Blad {
    Atom(String),
    Keyword(Keyword),
    Lambda(Environment, Box<Blad>, Box<Blad>),
    List(Vec<Blad>),
    Literal(Literal),
    Macro(Box<Blad>, Box<Blad>),
    Quote(Box<Blad>),
    Screech(Screech),
    Symbol(String),
    Unit,
}

impl Blad {
    pub fn get_atom(&self) -> Result<&str, Error> {
        match self {
            Blad::Atom(s) => Ok(s),
            _ => Err(Error::ExpectedAtom(self.clone())),
        }
    }

    pub fn get_list(&self) -> Result<&[Blad], Error> {
        match self {
            Blad::List(list) => Ok(list),
            _ => Err(Error::ExpectedList(self.clone())),
        }
    }

    pub fn get_symbol(&self) -> Result<&str, Error> {
        match self {
            Blad::Symbol(symbol) => Ok(symbol),
            _ => Err(Error::ExpectedSymbol(self.clone())),
        }
    }

    pub fn get_module(&self) -> Result<usize, Error> {
        match self {
            Blad::Screech(Screech::Module(id)) => Ok(*id),
            _ => Err(Error::ExpectedScreechModule(self.clone())),
        }
    }

    pub fn get_signal(&self) -> Result<Signal, Error> {
        match self {
            Blad::Screech(Screech::Signal(signal)) => Ok(*signal),
            _ => Err(Error::ExpectedScreechSignal(self.clone())),
        }
    }

    pub fn get_usize(&self) -> Result<usize, Error> {
        match self {
            Blad::Literal(Literal::Usize(int)) => Ok(*int),
            _ => Err(Error::ExpectedUsize(self.clone())),
        }
    }

    pub fn get_string(&self) -> Result<&str, Error> {
        match self {
            Blad::Literal(Literal::String(s)) => Ok(s),
            _ => Err(Error::ExpectedString(self.clone())),
        }
    }

    pub fn get_f32(&self) -> Result<f32, Error> {
        match self {
            Blad::Literal(Literal::F32(f)) => Ok(*f),
            _ => Err(Error::ExpectedF32(self.clone())),
        }
    }

    pub fn to_pitch(&self) -> Result<f32, Error> {
        match self {
            Blad::Atom(s) => atom_to_pitch(s).ok_or(Error::InvalidNote(s.into())),
            _ => Err(Error::ExpectedAtom(self.clone())),
        }
    }
}

impl PartialEq for Blad {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Blad::Atom(a), Blad::Atom(b)) => a == b,
            (Blad::Keyword(a), Blad::Keyword(b)) => a == b,
            (Blad::List(a), Blad::List(b)) => a == b,
            (Blad::Literal(a), Blad::Literal(b)) => a == b,
            (Blad::Quote(a), Blad::Quote(b)) => a == b,
            (Blad::Screech(a), Blad::Screech(b)) => a == b,
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
    Cast,
    Call,
    Samples,
    String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Screech {
    Module(usize),
    Signal(Signal),
}
