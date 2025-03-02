use super::{eval, parse, Environment};
use std::cell::RefCell;
use std::rc::Rc;

const PRELUDE: &'static str = "
    (let false 0)
    (let true 1)

    (let or (lambda (a b)
        (if a a b)))

    (let and (lambda (a b)
        (if a b a)))

    (let <= (lambda (a b)
        (or (< a b) (= a b))))

    (let >= (lambda (a b)
        (or (> a b) (= a b))))

    (let empty?
        (lambda (l) (if (head l) false true)))

    (let length (lambda (l) (do
        (let r (lambda (l i)
            (if (empty? l) i (r (tail l) (+ i 1)))
        ))

        (r l 0)
    )))
";

pub fn prelude_environment() -> Rc<RefCell<Environment>> {
    let env = Rc::new(RefCell::new(Environment::new()));

    let program = parse(PRELUDE).unwrap();

    for i in 0..program.len() {
        eval(&program[i], env.clone()).unwrap();
    }

    env
}

#[cfg(test)]
mod tests {
    use super::super::{parse, run_program, Blad, Literal};

    #[test]
    fn test_empty() {
        let program = parse("(empty? '())").unwrap();

        assert_eq!(
            run_program(&program).unwrap(),
            Blad::Literal(Literal::Usize(1)),
        );
    }

    #[test]
    fn test_non_empty() {
        let program = parse("(empty? '(1 2 3))").unwrap();

        assert_eq!(
            run_program(&program).unwrap(),
            Blad::Literal(Literal::Usize(0)),
        );
    }

    #[test]
    fn test_or() {
        assert_eq!(
            run_program(&parse("(or true true)").unwrap()).unwrap(),
            Blad::Literal(Literal::Usize(1)),
        );

        assert_eq!(
            run_program(&parse("(or true false)").unwrap()).unwrap(),
            Blad::Literal(Literal::Usize(1)),
        );

        assert_eq!(
            run_program(&parse("(or false true)").unwrap()).unwrap(),
            Blad::Literal(Literal::Usize(1)),
        );

        assert_eq!(
            run_program(&parse("(or false false)").unwrap()).unwrap(),
            Blad::Literal(Literal::Usize(0)),
        );
    }

    #[test]
    fn test_and() {
        assert_eq!(
            run_program(&parse("(and true true)").unwrap()).unwrap(),
            Blad::Literal(Literal::Usize(1)),
        );

        assert_eq!(
            run_program(&parse("(and true false)").unwrap()).unwrap(),
            Blad::Literal(Literal::Usize(0)),
        );

        assert_eq!(
            run_program(&parse("(and false true)").unwrap()).unwrap(),
            Blad::Literal(Literal::Usize(0)),
        );

        assert_eq!(
            run_program(&parse("(and false false)").unwrap()).unwrap(),
            Blad::Literal(Literal::Usize(0)),
        );
    }

    #[test]
    fn test_length() {
        let program = parse(
            "
            (length '(1 2 3 4))
        ",
        )
        .unwrap();

        assert_eq!(
            run_program(&program).unwrap(),
            Blad::Literal(Literal::Usize(4)),
        );
    }
}
