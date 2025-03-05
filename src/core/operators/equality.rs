use super::super::eval;
use crate::{Blad, BladError, Environment, Literal};

use std::cell::RefCell;
use std::rc::Rc;

pub fn process_equal(list: &[Blad], env: Rc<RefCell<Environment>>) -> Result<Blad, BladError> {
    let a = eval(&list[0], env.clone())?;
    let b = eval(&list[1], env.clone())?;

    match (&a, &b) {
        (Blad::Literal(Literal::Usize(x)), Blad::Literal(Literal::Usize(y))) => {
            let output = if x == y { 1 } else { 0 };

            Ok(Blad::Literal(Literal::Usize(output)))
        }
        (Blad::Unit, Blad::Unit) => Ok(Blad::Literal(Literal::Usize(1))),
        _ => Ok(Blad::Literal(Literal::Usize(0))),
    }
}
pub fn process_greater_than(
    list: &[Blad],
    env: Rc<RefCell<Environment>>,
) -> Result<Blad, BladError> {
    let a = eval(&list[0], env.clone())?;
    let b = eval(&list[1], env.clone())?;

    match (&a, &b) {
        (Blad::Literal(Literal::Usize(x)), Blad::Literal(Literal::Usize(y))) => {
            let output = if x > y { 1 } else { 0 };

            Ok(Blad::Literal(Literal::Usize(output)))
        }
        _ => Err(BladError::ExpectedSameTypes(a, b)),
    }
}

pub fn process_less_than(list: &[Blad], env: Rc<RefCell<Environment>>) -> Result<Blad, BladError> {
    let a = eval(&list[0], env.clone())?;
    let b = eval(&list[1], env.clone())?;

    match (&a, &b) {
        (Blad::Literal(Literal::Usize(x)), Blad::Literal(Literal::Usize(y))) => {
            let output = if x < y { 1 } else { 0 };

            Ok(Blad::Literal(Literal::Usize(output)))
        }
        _ => Err(BladError::ExpectedSameTypes(a, b)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::run;

    #[test]
    fn boolean() {
        assert_eq!(
            run("(= true false)").unwrap(),
            Blad::Literal(Literal::Usize(0)),
        );

        assert_eq!(
            run("(= true true)").unwrap(),
            Blad::Literal(Literal::Usize(1)),
        );

        assert_eq!(
            run("(= false false)").unwrap(),
            Blad::Literal(Literal::Usize(1)),
        );
    }

    #[test]
    fn numeric() {
        assert_eq!(run("(= 42 42)").unwrap(), Blad::Literal(Literal::Usize(1)));
        assert_eq!(run("(= 42 7)").unwrap(), Blad::Literal(Literal::Usize(0)));
    }

    #[test]
    fn greater_than() {
        assert_eq!(run("(> 42 42)").unwrap(), Blad::Literal(Literal::Usize(0)));
        assert_eq!(run("(> 42 7)").unwrap(), Blad::Literal(Literal::Usize(1)));
    }

    #[test]
    fn less_than() {
        assert_eq!(run("(< 7 7)").unwrap(), Blad::Literal(Literal::Usize(0)));
        assert_eq!(run("(< 7 42)").unwrap(), Blad::Literal(Literal::Usize(1)));
    }

    #[test]
    fn unit() {
        assert_eq!(
            run("(= '() '())").unwrap(),
            Blad::Literal(Literal::Usize(1))
        );
    }
}
