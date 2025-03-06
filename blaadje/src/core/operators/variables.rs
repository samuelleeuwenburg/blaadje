use super::super::eval;
use crate::{Blad, BladError, Environment};

use std::cell::RefCell;
use std::rc::Rc;

pub fn process_let(list: &[Blad], env: Rc<RefCell<Environment>>) -> Result<Blad, BladError> {
    let symbol = &list[0];
    let value = &list[1];

    match (symbol, value) {
        (Blad::Symbol(key), value) => {
            let result = eval(value, env.clone())?;
            env.borrow_mut().set(key, result)?;
            Ok(Blad::Unit)
        }
        _ => Err(BladError::ExpectedSymbol(symbol.clone())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{run, Literal};

    #[test]
    fn definitions() {
        assert_eq!(
            run("
                (do
                    (let x 11)
                    (let y 23)
                    (let z 32)
                    (+ x y z)
                )
            ")
            .unwrap(),
            Blad::Literal(Literal::Usize(66))
        );
    }

    #[test]
    fn immutability() {
        assert!(matches!(
            run("
                (do
                    (let x 11)
                    (let x 23)
                )
            ")
            .unwrap_err(),
            BladError::AttemptToRedefineVariable(_),
        ));
    }

    #[test]
    fn allow_shadowing() {
        assert_eq!(
            run("
                (do
                    (let x 1)
                    (let foo (fn (x) (+ x 10)))
                    (foo 2)
                )
            ")
            .unwrap(),
            Blad::Literal(Literal::Usize(12))
        );
    }
}
