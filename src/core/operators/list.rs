use super::super::eval;
use crate::{Blad, BladError, Environment};

use std::cell::RefCell;
use std::rc::Rc;

pub fn process_list(list: &[Blad], env: Rc<RefCell<Environment>>) -> Result<Blad, BladError> {
    if list.len() == 0 {
        return Ok(Blad::Unit);
    }

    let result = list
        .iter()
        .map(|b| eval(b, env.clone()))
        .collect::<Result<_, _>>()?;

    Ok(Blad::List(result))
}

pub fn process_do(list: &[Blad], env: Rc<RefCell<Environment>>) -> Result<Blad, BladError> {
    match process_list(list, env)? {
        Blad::List(b) => Ok(b.last().unwrap_or(&Blad::Unit).clone()),
        blad => Ok(blad),
    }
}

pub fn process_head(list: &[Blad], env: Rc<RefCell<Environment>>) -> Result<Blad, BladError> {
    let result = eval(&list[0], env.clone())?;
    match result {
        Blad::Unit => Ok(Blad::Unit),
        Blad::List(l) => Ok(l[0].clone()),
        _ => Err(BladError::ExpectedList(result)),
    }
}

pub fn process_tail(list: &[Blad], env: Rc<RefCell<Environment>>) -> Result<Blad, BladError> {
    let result = eval(&list[0], env.clone())?;

    match result {
        Blad::Unit => Ok(Blad::Unit),
        Blad::List(ref list) if list.len() < 2 => Ok(Blad::Unit),
        Blad::List(list) => {
            let mut tail = vec![];
            tail.extend_from_slice(&list[1..list.len()]);

            Ok(Blad::List(tail))
        }
        _ => Err(BladError::ExpectedList(result)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{run, Literal};

    #[test]
    fn list() {
        assert_eq!(
            run("(list 1 2 3 4)").unwrap(),
            Blad::List(vec![
                Blad::Literal(Literal::Usize(1)),
                Blad::Literal(Literal::Usize(2)),
                Blad::Literal(Literal::Usize(3)),
                Blad::Literal(Literal::Usize(4)),
            ])
        );
    }

    #[test]
    fn list_empty() {
        assert_eq!(run("(list)").unwrap(), Blad::Unit);
    }

    #[test]
    fn do_test() {
        assert_eq!(
            run("
                (do
                    (let x 2)
                    (let y 4)
                    (+ x y)
                )
            ")
            .unwrap(),
            Blad::Literal(Literal::Usize(6))
        );
    }

    #[test]
    fn do_empty() {
        assert_eq!(run("(do)").unwrap(), Blad::Unit);
    }

    #[test]
    fn head() {
        assert_eq!(
            run("(head '(1 2 3 4))").unwrap(),
            Blad::Literal(Literal::Usize(1))
        );
    }

    #[test]
    fn head_empty() {
        assert_eq!(run("(head '())").unwrap(), Blad::Unit);
    }

    #[test]
    fn tail() {
        assert_eq!(
            run("(tail '(1 2 3 4))").unwrap(),
            Blad::List(vec![
                Blad::Literal(Literal::Usize(2)),
                Blad::Literal(Literal::Usize(3)),
                Blad::Literal(Literal::Usize(4)),
            ])
        );
    }

    #[test]
    fn tail_empty() {
        assert_eq!(run("(tail '())").unwrap(), Blad::Unit);
    }
}
