use super::super::{args, eval};
use super::variables::resolve_lets;
use crate::{Blad, Environment, Error};
use std::sync::{Arc, Mutex};

pub fn process_macro(list: &[Blad], _env: Arc<Mutex<Environment>>) -> Result<Blad, Error> {
    args(list, 2)?;

    let params = &list[0];
    let body = &list[1];

    match params {
        Blad::Unit | Blad::List(_) | Blad::Symbol(_) => Ok(Blad::Macro(
            Box::new(params.clone()),
            Box::new(body.clone()),
        )),
        _ => Err(Error::IncorrectMacroSyntax(Blad::List(list.to_vec()))),
    }
}

pub fn expand_macro_call(
    params: &Blad,
    body: &Blad,
    list: &[Blad],
    env: Arc<Mutex<Environment>>,
) -> Result<Blad, Error> {
    let inner_env = Arc::new(Mutex::new(Environment::child_from(env.clone())));
    let values = Blad::List(list.to_vec());

    resolve_lets(params, &values, inner_env.clone())?;

    eval(&body, inner_env)
}

pub fn process_macro_call(
    params: &Blad,
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
