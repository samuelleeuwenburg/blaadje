use super::super::{args, eval};
use crate::{Blad, Environment, Error, Literal};

use std::cell::RefCell;
use std::rc::Rc;

pub fn process_if(list: &[Blad], env: Rc<RefCell<Environment>>) -> Result<Blad, Error> {
    args(list, 3)?;

    let condition = eval(&list[0], env.clone())?;
    let right = &list[1];
    let left = &list[2];

    match condition {
        Blad::Unit => eval(left, env.clone()),
        Blad::Literal(Literal::Usize(0)) => eval(left, env.clone()),
        _ => eval(right, env.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::run;

    #[test]
    fn truthy() {
        assert_eq!(
            run("(if (= 42 42) 12 6)").unwrap(),
            Blad::Literal(Literal::Usize(12)),
        );
    }

    #[test]
    fn falsy() {
        assert_eq!(
            run("(if (= 42 7) 12 6)").unwrap(),
            Blad::Literal(Literal::Usize(6)),
        );
    }
}
