use super::operators::{
    process_add, process_do, process_equal, process_greater_than, process_head, process_if,
    process_lambda, process_lambda_call, process_less_than, process_let, process_list,
    process_subtract, process_tail,
};
use super::{Blad, BladError, Environment, Keyword};
use std::cell::RefCell;
use std::rc::Rc;

pub fn eval(program: &Blad, env: Rc<RefCell<Environment>>) -> Result<Blad, BladError> {
    match program {
        Blad::Unit => Ok(program.clone()),
        Blad::Literal(_) => Ok(program.clone()),
        Blad::Quote(blad) => Ok(*blad.clone()),
        Blad::List(ref list) if list.is_empty() => Ok(Blad::Unit),
        Blad::List(list) => {
            let operator = eval(list.get(0).unwrap(), env.clone())?;
            let rest = &list[1..list.len()];

            match &operator {
                Blad::Keyword(keyword) => match keyword {
                    Keyword::Let => process_let(rest, env.clone()),
                    Keyword::List => process_list(rest, env.clone()),
                    Keyword::Head => process_head(rest, env.clone()),
                    Keyword::Tail => process_tail(rest, env.clone()),
                    Keyword::Do => process_do(rest, env.clone()),
                    Keyword::Add => process_add(rest, env.clone()),
                    Keyword::Subtract => process_subtract(rest, env.clone()),
                    Keyword::Lambda => process_lambda(rest, env.clone()),
                    Keyword::If => process_if(rest, env.clone()),
                    Keyword::Equal => process_equal(rest, env.clone()),
                    Keyword::GreaterThan => process_greater_than(rest, env.clone()),
                    Keyword::LessThan => process_less_than(rest, env.clone()),
                },
                Blad::Lambda(closure, params, body) => {
                    process_lambda_call(closure, params, body, rest, env.clone())
                }
                _ => Err(BladError::ExpectedProcedure(operator)),
            }
        }
        Blad::Lambda(closure, params, body) => {
            Ok(Blad::Lambda(closure.clone(), params.clone(), body.clone()))
        }
        Blad::Keyword(string) => Ok(Blad::Keyword(string.clone())),
        Blad::Symbol(string) => match string.as_str() {
            "+" => Ok(Blad::Keyword(Keyword::Add)),
            "-" => Ok(Blad::Keyword(Keyword::Subtract)),
            "<" => Ok(Blad::Keyword(Keyword::LessThan)),
            "=" => Ok(Blad::Keyword(Keyword::Equal)),
            ">" => Ok(Blad::Keyword(Keyword::GreaterThan)),
            "do" => Ok(Blad::Keyword(Keyword::Do)),
            "head" => Ok(Blad::Keyword(Keyword::Head)),
            "if" => Ok(Blad::Keyword(Keyword::If)),
            "fn" => Ok(Blad::Keyword(Keyword::Lambda)),
            "let" => Ok(Blad::Keyword(Keyword::Let)),
            "list" => Ok(Blad::Keyword(Keyword::List)),
            "tail" => Ok(Blad::Keyword(Keyword::Tail)),
            _ => env
                .borrow()
                .get(string)
                .ok_or(BladError::UndefinedSymbol(string.clone())),
        },
    }
}
