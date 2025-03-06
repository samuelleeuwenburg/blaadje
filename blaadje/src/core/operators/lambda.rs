use super::super::eval;
use crate::{Blad, BladError, Environment};

use std::cell::RefCell;
use std::rc::Rc;

pub fn process_lambda(list: &[Blad], env: Rc<RefCell<Environment>>) -> Result<Blad, BladError> {
    let params = &list[0];
    let body = &list[1];

    match (params, list.len()) {
        (Blad::List(p), 2) => {
            let mut param_strings = vec![];
            let closure = Environment::child_from(env.clone());

            for blad in p.into_iter() {
                if let Blad::Symbol(param) = blad {
                    param_strings.push(param.clone());
                } else {
                    return Err(BladError::ExpectedSymbol(blad.clone()));
                }
            }

            Ok(Blad::Lambda(closure, param_strings, Box::new(body.clone())))
        }
        (_, 2) => Err(BladError::ExpectedList(params.clone())),
        _ => Err(BladError::IncorrectLambdaSyntax(Blad::List(list.to_vec()))),
    }
}

pub fn process_lambda_call(
    closure: &Environment,
    params: &Vec<String>,
    body: &Blad,
    list: &[Blad],
    env: Rc<RefCell<Environment>>,
) -> Result<Blad, BladError> {
    if params.len() != list.len() {
        return Err(BladError::WrongNumberOfArguments(params.len(), list.len()));
    }

    let inner_env = Rc::new(RefCell::new(closure.clone()));

    for (i, p) in params.iter().enumerate() {
        let result = eval(&list[i], env.clone())?;
        inner_env.borrow_mut().set(p, result)?;
    }

    eval(&body, inner_env)
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
