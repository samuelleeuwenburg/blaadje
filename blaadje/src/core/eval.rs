use super::operators::{
    process_add, process_append, process_call, process_cast, process_cons, process_do,
    process_equal, process_greater_than, process_head, process_if, process_lambda,
    process_lambda_call, process_less_than, process_let, process_list, process_macro,
    process_macro_call, process_subtract, process_tail,
};
use super::{Blad, Environment, Error, Keyword};
use std::sync::{Arc, Mutex};

pub fn eval(program: &Blad, env: Arc<Mutex<Environment>>) -> Result<Blad, Error> {
    match program {
        Blad::Unit
        | Blad::Atom(_)
        | Blad::Literal(_)
        | Blad::Keyword(_)
        | Blad::Screech(_)
        | Blad::Lambda(_, _, _)
        | Blad::Macro(_, _) => Ok(program.clone()),
        Blad::Quote(blad) => Ok(*blad.clone()),
        Blad::Symbol(string) => {
            let env = env.lock().unwrap();
            env.get(string)
                .ok_or(Error::UndefinedSymbol(string.clone()))
        }
        Blad::List(ref list) if list.is_empty() => Ok(Blad::Unit),
        Blad::List(list) => {
            let operator = eval(list.get(0).unwrap(), env.clone())?;
            let rest = &list[1..list.len()];

            match &operator {
                Blad::Keyword(keyword) => match keyword {
                    Keyword::Add => process_add(rest, env.clone()),
                    Keyword::Append => process_append(rest, env.clone()),
                    Keyword::Cons => process_cons(rest, env.clone()),
                    Keyword::Call => process_call(rest, env.clone()),
                    Keyword::Cast => process_cast(rest, env.clone()),
                    Keyword::Macro => process_macro(rest, env.clone()),
                    Keyword::Do => process_do(rest, env.clone()),
                    Keyword::Equal => process_equal(rest, env.clone()),
                    Keyword::GreaterThan => process_greater_than(rest, env.clone()),
                    Keyword::Head => process_head(rest, env.clone()),
                    Keyword::If => process_if(rest, env.clone()),
                    Keyword::Lambda => process_lambda(rest, env.clone()),
                    Keyword::LessThan => process_less_than(rest, env.clone()),
                    Keyword::Let => process_let(rest, env.clone()),
                    Keyword::List => process_list(rest, env.clone()),
                    Keyword::Subtract => process_subtract(rest, env.clone()),
                    Keyword::Tail => process_tail(rest, env.clone()),
                },
                Blad::Lambda(closure, params, body) => {
                    process_lambda_call(closure, params, body, rest, env.clone())
                }
                Blad::Macro(params, body) => process_macro_call(params, body, rest, env.clone()),
                _ => Err(Error::ExpectedProcedure(operator)),
            }
        }
    }
}

pub fn args(list: &[Blad], args: usize) -> Result<&[Blad], Error> {
    if list.len() != args {
        Err(Error::IncorrectNumberOfArguments(list.len(), args))
    } else {
        Ok(list)
    }
}

pub fn args_min(list: &[Blad], args: usize) -> Result<&[Blad], Error> {
    if list.len() < args {
        Err(Error::IncorrectNumberOfArguments(list.len(), args))
    } else {
        Ok(list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::run;

    #[test]
    fn quote_keywords() {
        assert_eq!(
            run("(list 'let)").unwrap(),
            Blad::List(vec![Blad::Keyword(Keyword::Let)])
        );

        assert_eq!(
            run("(list let)").unwrap(),
            Blad::List(vec![Blad::Keyword(Keyword::Let)])
        );
    }
}
