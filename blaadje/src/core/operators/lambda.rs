use super::super::{args, eval};
use crate::{Blad, Environment, Error};
use std::sync::{Arc, Mutex};

pub fn process_lambda(list: &[Blad], env: Arc<Mutex<Environment>>) -> Result<Blad, Error> {
    args(list, 2)?;

    let params = &list[0];
    let body = &list[1];

    let closure = Environment::child_from(env.clone());

    match params {
        Blad::Unit => Ok(Blad::Lambda(closure, vec![], Box::new(body.clone()))),
        Blad::List(p) => {
            let mut param_strings = vec![];

            for blad in p.into_iter() {
                if let Blad::Symbol(param) = blad {
                    param_strings.push(param.clone());
                } else {
                    return Err(Error::ExpectedSymbol(blad.clone()));
                }
            }

            Ok(Blad::Lambda(closure, param_strings, Box::new(body.clone())))
        }
        _ => Err(Error::IncorrectLambdaSyntax(Blad::List(list.to_vec()))),
    }
}

pub fn process_lambda_call(
    closure: &Environment,
    params: &Vec<String>,
    body: &Blad,
    list: &[Blad],
    env: Arc<Mutex<Environment>>,
) -> Result<Blad, Error> {
    args(list, params.len())?;

    let mut inner_env = closure.clone();

    for (i, p) in params.iter().enumerate() {
        let result = eval(&list[i], env.clone())?;
        inner_env.set(p, result)?;
    }

    eval(&body, Arc::new(Mutex::new(inner_env)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{run, Literal};

    #[test]
    fn definition() {
        assert_eq!(
            run("
                (do
                    (let add_one (fn (x) (+ x 1)))
                    (add_one 1)
                )
            ")
            .unwrap(),
            Blad::Literal(Literal::Usize(2)),
        );
    }

    #[test]
    fn curry() {
        assert_eq!(
            run("
                (do
                    (let adder
                        (fn (x)
                            (fn (y) (+ x y))
                        )
                    )

                    (let add_one (adder 1))
                    (let add_two (adder 2))

                    (+ (add_one 2) (add_two 1))
                )
            ")
            .unwrap(),
            Blad::Literal(Literal::Usize(6)),
        );
    }

    #[test]
    fn recursion() {
        assert_eq!(
            run("
                (do
                    (let fibonacci (fn (x)
                        (if (< x 2)
                            x
                            (+ (fibonacci (- x 2)) (fibonacci (- x 1)))
                        )
                    ))

                    (fibonacci 12)
                )
            ")
            .unwrap(),
            Blad::Literal(Literal::Usize(144)),
        );
    }

    #[test]
    fn tail_recursion() {
        assert_eq!(
            run("
                (do
                    (let fibonacci (fn (n p1 p2)
                        (if (< n 2)
                            p2
                            (fibonacci (- n 1) (+ p2 p1) p1)
                        )
                    ))

                    (fibonacci 64 1 1)
                )
            ")
            .unwrap(),
            Blad::Literal(Literal::Usize(10610209857723)),
        );
    }

    #[test]
    fn direct_call() {
        assert_eq!(
            run("((fn (x) (+ x 1)) 1)").unwrap(),
            Blad::Literal(Literal::Usize(2)),
        );
    }
}
