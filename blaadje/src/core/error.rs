use super::Blad;
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
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
    IncorrectArguments(Vec<Error>),
    IncorrectLambdaSyntax(Blad),
    IncorrectMacroSyntax(Blad),
    IncorrectNumberOfArguments(usize, usize),
    IncorrectPropertyPair(String, Blad),
    InvalidProperty(String),
    InvalidToken(String),
    ModuleNotFound(usize),
    ModuleAtomNotFound(String),
    ParseError(usize),
    UndefinedOperator(String),
    UndefinedSymbol(String),
    UnexpectedToken(String),
    UnknownModule(String),
    UnsupportedNumericType(String),
    InvalidNote(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None
    }
}
