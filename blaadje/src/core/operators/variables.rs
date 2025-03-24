use super::super::{args, eval};
use crate::{Blad, Environment, Error};
use std::sync::{Arc, Mutex};

pub fn process_let(list: &[Blad], env: Arc<Mutex<Environment>>) -> Result<Blad, Error> {
    args(list, 2)?;

    let key = &list[0].get_symbol()?;
    let value = &list[1];

    let result = eval(value, env.clone())?;
    let mut env = env.lock().unwrap();
    env.set(key, result)?;

    Ok(Blad::Unit)
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
            Error::AttemptToRedefineVariable(_),
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
