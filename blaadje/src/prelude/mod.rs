use crate::{eval, parse, Environment, Error};
use std::cell::RefCell;
use std::rc::Rc;

const PRELUDE: &'static str = "
    (let false 0)
    (let true 1)

    (let or (fn (a b) (if a a b)))

    (let and (fn (a b) (if a b a)))

    (let <= (fn (a b) (or (< a b) (= a b))))

    (let >= (fn (a b) (or (> a b) (= a b))))

    (let empty? (fn (l) (if (= (head l) '()) true false)))

    (let length (fn (items) (do
        (let f (fn (items index)
            (if (empty? items) index (f (tail items) (+ index 1)))
        ))

        (f items 0)
    )))

    (let fold (fn (items initial_value function) (do
        (if (empty? items)
            initial_value
            (do
                (let result (function initial_value (head items)))
                (fold (tail items) result function)
            )
        )
    )))

    (let map (fn (items f)
        (fold items '() (fn (xs x)
            (append (f x) xs)
        ))
    ))

    (let filter (fn (items f)
        (fold items '() (fn (xs x)
            (if (f x) (append x xs) xs)
        ))
    ))
";

pub fn set_prelude(env: Rc<RefCell<Environment>>) -> Result<Rc<RefCell<Environment>>, Error> {
    let program = parse(PRELUDE)?;
    eval(&program, env.clone())?;

    Ok(env)
}

#[cfg(test)]
mod tests {
    use crate::{run, Blad, Literal};

    #[test]
    fn empty() {
        assert_eq!(
            run("(empty? '())").unwrap(),
            Blad::Literal(Literal::Usize(1)),
        );
    }

    #[test]
    fn non_empty() {
        assert_eq!(
            run("(empty? '(1 2 3))").unwrap(),
            Blad::Literal(Literal::Usize(0)),
        );
    }

    #[test]
    fn or() {
        assert_eq!(
            run("(or true true)").unwrap(),
            Blad::Literal(Literal::Usize(1)),
        );

        assert_eq!(
            run("(or true false)").unwrap(),
            Blad::Literal(Literal::Usize(1)),
        );

        assert_eq!(
            run("(or false true)").unwrap(),
            Blad::Literal(Literal::Usize(1)),
        );

        assert_eq!(
            run("(or false false)").unwrap(),
            Blad::Literal(Literal::Usize(0)),
        );
    }

    #[test]
    fn and() {
        assert_eq!(
            run("(and true true)").unwrap(),
            Blad::Literal(Literal::Usize(1)),
        );

        assert_eq!(
            run("(and true false)").unwrap(),
            Blad::Literal(Literal::Usize(0)),
        );

        assert_eq!(
            run("(and false true)").unwrap(),
            Blad::Literal(Literal::Usize(0)),
        );

        assert_eq!(
            run("(and false false)").unwrap(),
            Blad::Literal(Literal::Usize(0)),
        );
    }

    #[test]
    fn length() {
        assert_eq!(
            run("(length '(1 2 3 4))").unwrap(),
            Blad::Literal(Literal::Usize(4)),
        );
    }

    #[test]
    fn fold() {
        assert_eq!(
            run("(fold '(1 2 3 4) 0 +)").unwrap(),
            Blad::Literal(Literal::Usize(10)),
        );
    }

    #[test]
    fn map() {
        assert_eq!(
            run("(map '(0 1 2 3) (fn (x) (+ x 1)))").unwrap(),
            Blad::List(vec![
                Blad::Literal(Literal::Usize(1)),
                Blad::Literal(Literal::Usize(2)),
                Blad::Literal(Literal::Usize(3)),
                Blad::Literal(Literal::Usize(4)),
            ])
        );
    }

    #[test]
    fn filter() {
        assert_eq!(
            run("(filter '(8 2 6 3) (fn (x) (> x 4)))").unwrap(),
            Blad::List(vec![
                Blad::Literal(Literal::Usize(8)),
                Blad::Literal(Literal::Usize(6)),
            ])
        );
    }
}
