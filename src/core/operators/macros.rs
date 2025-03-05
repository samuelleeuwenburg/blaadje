use super::super::eval;
use crate::{Blad, BladError, Environment};

use std::cell::RefCell;
use std::rc::Rc;

pub fn process_macro(list: &[Blad], _env: Rc<RefCell<Environment>>) -> Result<Blad, BladError> {
    let params = &list[0];
    let body = &list[1];

    match (params, list.len()) {
        (Blad::List(p), 2) => {
            let mut param_strings = vec![];

            for blad in p.into_iter() {
                if let Blad::Symbol(param) = blad {
                    param_strings.push(param.clone());
                } else {
                    return Err(BladError::ExpectedSymbol(blad.clone()));
                }
            }

            Ok(Blad::Macro(param_strings, Box::new(body.clone())))
        }
        (_, 2) => Err(BladError::ExpectedList(params.clone())),
        _ => Err(BladError::IncorrectMacroSyntax(Blad::List(list.to_vec()))),
    }
}

pub fn expand_macro_call(
    params: &Vec<String>,
    body: &Blad,
    list: &[Blad],
    env: Rc<RefCell<Environment>>,
) -> Result<Blad, BladError> {
    if params.len() != list.len() {
        return Err(BladError::WrongNumberOfArguments(params.len(), list.len()));
    }

    let inner_env = Rc::new(RefCell::new(Environment::child_from(env.clone())));

    for (i, p) in params.iter().enumerate() {
        inner_env.borrow_mut().set(p, list[i].clone())?;
    }

    eval(&body, inner_env)
}

pub fn process_macro_call(
    params: &Vec<String>,
    body: &Blad,
    list: &[Blad],
    env: Rc<RefCell<Environment>>,
) -> Result<Blad, BladError> {
    let output = expand_macro_call(params, body, list, env.clone())?;
    eval(&output, env)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{run, Literal};

    #[test]
    fn infix() {
        assert_eq!(
            run("
                (do
                    (let infix (macro (a b c) (list b a c)))

                    (infix 1 + 1)
                )
            ")
            .unwrap(),
            Blad::Literal(Literal::Usize(2))
        );
    }

    #[test]
    fn let_two() {
        assert_eq!(
            run("
                (do
                    (let let_two (macro (key_a key_b value)
                        (list 'do
                            (list 'let key_a value)
                            (list 'let key_b value)
                        )
                    ))

                    (let_two x y 8)
                    (+ x y)
                )
            ")
            .unwrap(),
            Blad::Literal(Literal::Usize(16))
        );
    }
}
