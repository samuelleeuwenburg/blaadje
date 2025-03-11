use super::super::{args_min, eval};
use crate::{Blad, Environment, Error, Literal};

use std::cell::RefCell;
use std::rc::Rc;

pub fn process_add(list: &[Blad], env: Rc<RefCell<Environment>>) -> Result<Blad, Error> {
    args_min(list, 1)?;

    let result: Vec<Blad> = list
        .iter()
        .map(|b| eval(b, env.clone()))
        .collect::<Result<_, _>>()?;

    match result[0] {
        Blad::Literal(Literal::Usize(_)) => {
            let sum = get_usizes(&result)?.iter().sum();
            Ok(Blad::Literal(Literal::Usize(sum)))
        }
        Blad::Literal(Literal::F32(_)) => {
            let sum = get_floats(&result)?.iter().sum();
            Ok(Blad::Literal(Literal::F32(sum)))
        }
        _ => Err(Error::ExpectedNumber(result[0].clone())),
    }
}

pub fn process_subtract(list: &[Blad], env: Rc<RefCell<Environment>>) -> Result<Blad, Error> {
    args_min(list, 1)?;

    let result: Vec<Blad> = list
        .iter()
        .map(|b| eval(b, env.clone()))
        .collect::<Result<_, _>>()?;

    match result[0] {
        Blad::Literal(Literal::Usize(_)) => {
            let nums = get_usizes(&result)?;
            let sum = nums.iter().skip(1).fold(nums[0], |acc, &x| acc - x);
            Ok(Blad::Literal(Literal::Usize(sum)))
        }
        Blad::Literal(Literal::F32(_)) => {
            let nums = get_floats(&result)?;
            let sum = nums.iter().skip(1).fold(nums[0], |acc, &x| acc - x);
            Ok(Blad::Literal(Literal::F32(sum)))
        }
        _ => Err(Error::ExpectedNumber(result[0].clone())),
    }
}

fn get_usizes(list: &[Blad]) -> Result<Vec<usize>, Error> {
    let mut nums = vec![];

    for b in list {
        match b {
            Blad::Literal(Literal::Usize(x)) => nums.push(*x),
            _ => return Err(Error::ExpectedUsize(b.clone())),
        }
    }

    Ok(nums)
}

fn get_floats(list: &[Blad]) -> Result<Vec<f32>, Error> {
    let mut nums = vec![];

    for b in list {
        match b {
            Blad::Literal(Literal::F32(x)) => nums.push(*x),
            _ => return Err(Error::ExpectedUsize(b.clone())),
        }
    }

    Ok(nums)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::run;

    #[test]
    fn addition() {
        assert_eq!(run("(+ 0 2)").unwrap(), Blad::Literal(Literal::Usize(2)));
        assert_eq!(run("(+ 1 2)").unwrap(), Blad::Literal(Literal::Usize(3)));
        assert_eq!(
            run("(+ 0 0 0 0 0 0 0 0 0)").unwrap(),
            Blad::Literal(Literal::Usize(0))
        );

        assert_eq!(
            run("(+ 1 2 3 4 5 6 7 8 9 10)").unwrap(),
            Blad::Literal(Literal::Usize(55))
        );
    }

    #[test]
    fn subtraction() {
        assert_eq!(
            run("(- 100 58)").unwrap(),
            Blad::Literal(Literal::Usize(42))
        );

        assert_eq!(
            run("(- 55555 4444 1111)").unwrap(),
            Blad::Literal(Literal::Usize(50000))
        );

        assert_eq!(
            run("(- 0 0 0 0)").unwrap(),
            Blad::Literal(Literal::Usize(0))
        );
    }
}
