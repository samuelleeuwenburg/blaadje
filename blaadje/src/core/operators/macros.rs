use super::super::{args, eval};
use crate::{Blad, Environment, Error};
use std::sync::{Arc, Mutex};

pub fn process_macro(list: &[Blad], _env: Arc<Mutex<Environment>>) -> Result<Blad, Error> {
    args(list, 2)?;

    let params = &list[0];
    let body = &list[1];

    match (params, list.len()) {
        (Blad::List(p), 2) => {
            let mut param_strings = vec![];

            for blad in p.into_iter() {
                if let Blad::Symbol(param) = blad {
                    param_strings.push(param.clone());
                } else {
                    return Err(Error::ExpectedSymbol(blad.clone()));
                }
            }

            Ok(Blad::Macro(param_strings, Box::new(body.clone())))
        }
        (_, 2) => Err(Error::ExpectedList(params.clone())),
        _ => Err(Error::IncorrectMacroSyntax(Blad::List(list.to_vec()))),
    }
}

pub fn expand_macro_call(
    params: &Vec<String>,
    body: &Blad,
    list: &[Blad],
    env: Arc<Mutex<Environment>>,
) -> Result<Blad, Error> {
    args(list, params.len())?;

    let mut inner_env = Environment::child_from(env.clone());

    for (i, p) in params.iter().enumerate() {
        inner_env.set(p, list[i].clone())?;
    }

    eval(&body, Arc::new(Mutex::new(inner_env)))
}

pub fn process_macro_call(
    params: &Vec<String>,
    body: &Blad,
    list: &[Blad],
    env: Arc<Mutex<Environment>>,
) -> Result<Blad, Error> {
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
