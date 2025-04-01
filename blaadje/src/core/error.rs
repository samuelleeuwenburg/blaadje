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
    ExpectedString(Blad),
    ExpectedSymbol(Blad),
    ExpectedUsize(Blad),
    FileError,
    IncorrectArguments(Vec<Error>),
    IncorrectLambdaSyntax(Blad),
    IncorrectMacroSyntax(Blad),
    IncorrectNumberOfArguments(usize, usize),
    IncorrectPropertyPair(String, Blad),
    IncorrectVariableDeclaration(Blad, Blad),
    IncorrectVariableDestructuring(usize, usize),
    InvalidNote(String),
    InvalidProperty(String),
    InvalidToken(String),
    ModuleIdNotFound(String),
    ModuleNotFound(usize),
    ParseError(usize),
    UnableToConvertToString(Blad),
    UndefinedOperator(String),
    UndefinedSymbol(String),
    UnexpectedToken(String),
    UnknownModule(String),
    UnsupportedNumericType(String),
    WavError,
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
