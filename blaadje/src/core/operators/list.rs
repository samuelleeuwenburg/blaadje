use super::super::{args, eval};
use crate::{Blad, Environment, Error, Literal};
use std::sync::{Arc, Mutex};

pub fn process_list(list: &[Blad], env: Arc<Mutex<Environment>>) -> Result<Blad, Error> {
    if list.len() == 0 {
        return Ok(Blad::Unit);
    }

    let result = list
        .iter()
        .map(|b| eval(b, env.clone()))
        .collect::<Result<_, _>>()?;

    Ok(Blad::List(result))
}

pub fn process_do(list: &[Blad], env: Arc<Mutex<Environment>>) -> Result<Blad, Error> {
    match process_list(list, env)? {
        Blad::List(b) => Ok(b.last().unwrap_or(&Blad::Unit).clone()),
        blad => Ok(blad),
    }
}

pub fn process_head(list: &[Blad], env: Arc<Mutex<Environment>>) -> Result<Blad, Error> {
    args(list, 1)?;

    let result = eval(&list[0], env.clone())?;
    match result {
        Blad::Unit => Ok(Blad::Unit),
        Blad::List(l) => Ok(l[0].clone()),
        _ => Err(Error::ExpectedList(result)),
    }
}

pub fn process_tail(list: &[Blad], env: Arc<Mutex<Environment>>) -> Result<Blad, Error> {
    args(list, 1)?;

    let result = eval(&list[0], env.clone())?;

    match result {
        Blad::Unit => Ok(Blad::Unit),
        Blad::List(ref list) if list.len() < 2 => Ok(Blad::Unit),
        Blad::List(list) => {
            let mut tail = vec![];
            tail.extend_from_slice(&list[1..list.len()]);

            Ok(Blad::List(tail))
        }
        _ => Err(Error::ExpectedList(result)),
    }
}

pub fn process_cons(list: &[Blad], env: Arc<Mutex<Environment>>) -> Result<Blad, Error> {
    args(list, 2)?;

    let item = eval(&list[0], env.clone())?;
    let items = eval(&list[1], env.clone())?;

    match items {
        Blad::Unit => Ok(Blad::List(vec![item])),
        Blad::List(l) => {
            let mut new_items = vec![item];
            new_items.extend_from_slice(&l);
            Ok(Blad::List(new_items))
        }
        _ => Err(Error::ExpectedList(items)),
    }
}

pub fn process_append(list: &[Blad], env: Arc<Mutex<Environment>>) -> Result<Blad, Error> {
    args(list, 2)?;

    let item = eval(&list[0], env.clone())?;
    let items = eval(&list[1], env.clone())?;

    match (&item, &items) {
        (Blad::Literal(Literal::String(a)), Blad::Literal(Literal::String(b))) => {
            let mut result = a.to_string();
            result.push_str(b);

            Ok(Blad::Literal(Literal::String(result)))
        }
        (_, Blad::List(l)) => {
            let mut new_items = l.clone();
            new_items.push(item);

            Ok(Blad::List(new_items))
        }
        (_, Blad::Unit) => Ok(Blad::List(vec![item])),
        _ => Err(Error::ExpectedList(items)),
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

    #[test]
    fn cons() {
        assert_eq!(
            run("(cons 1 '(2 3 4))").unwrap(),
            Blad::List(vec![
                Blad::Literal(Literal::Usize(1)),
                Blad::Literal(Literal::Usize(2)),
                Blad::Literal(Literal::Usize(3)),
                Blad::Literal(Literal::Usize(4)),
            ])
        );
    }

    #[test]
    fn cons_empty() {
        assert_eq!(
            run("(cons 1 '())").unwrap(),
            Blad::List(vec![Blad::Literal(Literal::Usize(1))])
        );
    }

    #[test]
    fn append() {
        assert_eq!(
            run("(append 1 '(2 3 4))").unwrap(),
            Blad::List(vec![
                Blad::Literal(Literal::Usize(2)),
                Blad::Literal(Literal::Usize(3)),
                Blad::Literal(Literal::Usize(4)),
                Blad::Literal(Literal::Usize(1)),
            ])
        );
    }

    #[test]
    fn append_string() {
        assert_eq!(
            run("(append \"foo\" \"bar\")").unwrap(),
            Blad::Literal(Literal::String("foobar".to_string())),
        );
    }

    #[test]
    fn append_empty() {
        assert_eq!(
            run("(append 1 '())").unwrap(),
            Blad::List(vec![Blad::Literal(Literal::Usize(1))])
        );
    }
}
