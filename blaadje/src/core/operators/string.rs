use super::super::{args, eval};
use crate::{Blad, Environment, Error, Literal};
use std::sync::{Arc, Mutex};

pub fn process_string(list: &[Blad], env: Arc<Mutex<Environment>>) -> Result<Blad, Error> {
    args(list, 1)?;

    let result = eval(&list[0], env.clone())?;

    let string = match &result {
        Blad::Atom(s) => Ok(s.clone()),
        Blad::Literal(Literal::String(s)) => Ok(s.clone()),
        Blad::Literal(Literal::F32(n)) => Ok(n.to_string()),
        Blad::Literal(Literal::Usize(n)) => Ok(n.to_string()),
        _ => Err(Error::UnableToConvertToString(result)),
    }?;

    Ok(Blad::Literal(Literal::String(string)))
}
