use super::super::{args, eval};
use crate::{Blad, Environment, Error};
use std::sync::{Arc, Mutex};

pub fn process_let(list: &[Blad], env: Arc<Mutex<Environment>>) -> Result<Blad, Error> {
    args(list, 2)?;

    let set = &list[0];
    let result = eval(&list[1], env.clone())?;

    resolve_lets(set, &result, env.clone())
}

fn resolve_lets(key: &Blad, value: &Blad, env: Arc<Mutex<Environment>>) -> Result<Blad, Error> {
    match (key, value) {
        (Blad::Symbol(key), _) => {
            let mut env = env.lock().unwrap();
            env.set(key, value.clone())?;

            Ok(Blad::Unit)
        }
        (Blad::List(keys), Blad::List(values)) => {
            if keys.len() != values.len() {
                return Err(Error::IncorrectVariableDestructuring(
                    keys.len(),
                    values.len(),
                ));
            }

            for (key, value) in keys.iter().zip(values.iter()) {
                resolve_lets(key, value, env.clone())?;
            }

            Ok(Blad::Unit)
        }
        _ => Err(Error::IncorrectVariableDeclaration(
            key.clone(),
            value.clone(),
        )),
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

    #[test]
    fn destructuring() {
        assert_eq!(
            run("
                (do
                    (let tuple (list (+ 1 2) (+ 10 20)))
                    (let (x y) tuple)
                    (+ x y)
                )
            ")
            .unwrap(),
            Blad::Literal(Literal::Usize(33))
        );
    }

    #[test]
    fn nested_destructuring() {
        assert_eq!(
            run("
                (do
                    (let nested (list (+ 1 2) (list 10 20)))
                    (let (x (y z)) nested)
                    (+ x y z)
                )
            ")
            .unwrap(),
            Blad::Literal(Literal::Usize(33))
        );
    }
}
