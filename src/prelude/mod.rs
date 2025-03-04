use crate::{eval, parse, BladError, Environment};
use std::cell::RefCell;
use std::rc::Rc;

const PRELUDE: &'static str = "
    (let false 0)
    (let true 1)

    (let or (fn (a b) (if a a b)))

    (let and (fn (a b) (if a b a)))

    (let <= (fn (a b) (or (< a b) (= a b))))

    (let >= (fn (a b) (or (> a b) (= a b))))

    (let empty? (fn (l) (if (head l) false true)))

    (let length (fn (l) (do
        (let r (fn (l i)
            (if (empty? l) i (r (tail l) (+ i 1)))
        ))

        (r l 0)
    )))
";

pub fn prelude_environment() -> Result<Rc<RefCell<Environment>>, BladError> {
    let env = Rc::new(RefCell::new(Environment::new()));

    let program = parse(PRELUDE)?;
    eval(&program, env.clone())?;

    println!("\n\n===========END OF PRELUDE==========\n\n");

    Ok(env)
}

#[cfg(test)]
mod tests {
    use crate::{run, Blad, Literal};

    #[test]
    fn test_empty() {
        assert_eq!(
            run("(empty? '())").unwrap(),
            Blad::Literal(Literal::Usize(1)),
        );
    }

    #[test]
    fn test_non_empty() {
        assert_eq!(
            run("(empty? '(1 2 3))").unwrap(),
            Blad::Literal(Literal::Usize(0)),
        );
    }

    #[test]
    fn test_or() {
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
    fn test_and() {
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

    // #[test]
    // fn test_length() {
    //     assert_eq!(
    //         run("(length '(1 2 3 4))").unwrap(),
    //         Blad::Literal(Literal::Usize(4)),
    //     );
    // }
}
