use crate::{eval, parse, Environment, Error};
use std::sync::{Arc, Mutex};

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

    (let nth (fn (items index)
        (if (<= index 0)
            (head items)
            (nth (tail items) (- index 1))
        )
    ))

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

    (let enumerate (fn (items) (do
        (let result (fold items '( () 0) (fn (acc item) (do
            (let items (nth acc 0))
            (let index (nth acc 1))
            (list
                (append (list index item) items)
                (+ index 1)
            )
        ))))
        (head result)
    )))
";

const SCREECH_PRELUDE: &'static str = "
    (let output_left (fn (signal)
        (call (list :output_left signal))))

    (let output_right (fn (signal)
        (call (list :output_right signal))))

    (let output (fn (signal)
        (do
            (output_left signal)
            (output_right signal)
        )
    ))

    (let output_disconnect_all (fn ()
        (call (list :output_disconnect_all))))

    (let scale (fn (signal scale)
        (call (list :scale signal scale))))

    (let offset (fn (signal offset)
        (call (list :offset signal offset))))

    (let set (fn (module property value)
        (call (list :set module (list property value)))))

    (let get (fn (module property)
        (call (list :get module property))))

    (let Osc.new (fn (id)
        (call (list :insert_module :oscillator id))))

    (let Midi.new (fn (id)
        (call (list :insert_module :midi id))))

    (let Vca.new (fn (id)
        (call (list :insert_module :vca id))))

    (let Filter.new (fn (id)
        (call (list :insert_module :filter id))))

    (let Sample.new (fn (id)
        (call (list :insert_module :sample id))))

    (let Clock.new (fn (id)
        (call (list :insert_module :clock id))))

    (let Sequencer.new (fn (id)
        (call (list :insert_module :sequencer id))))

    (let Module.new (fn (module id properties) (do
        (let m (module id))
        (map
            properties
            (fn (tuple) (set m (nth tuple 0) (nth tuple 1)))
        )
        m
    )))
";

pub fn set_prelude(env: Arc<Mutex<Environment>>) -> Result<Arc<Mutex<Environment>>, Error> {
    eval(&parse(PRELUDE)?, env.clone())?;
    eval(&parse(SCREECH_PRELUDE)?, env.clone())?;

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
    fn nth() {
        assert_eq!(
            run("(nth '(1 2 3 4) 0)").unwrap(),
            Blad::Literal(Literal::Usize(1)),
        );

        assert_eq!(
            run("(nth '(1 2 3 4) 2)").unwrap(),
            Blad::Literal(Literal::Usize(3)),
        );

        assert_eq!(run("(nth '(1 2 3 4) 100)").unwrap(), Blad::Unit);
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

    #[test]
    fn enumerate() {
        assert_eq!(
            run("(enumerate '(8 2 6 3))").unwrap(),
            Blad::List(vec![
                Blad::List(vec![
                    Blad::Literal(Literal::Usize(0)),
                    Blad::Literal(Literal::Usize(8)),
                ]),
                Blad::List(vec![
                    Blad::Literal(Literal::Usize(1)),
                    Blad::Literal(Literal::Usize(2)),
                ]),
                Blad::List(vec![
                    Blad::Literal(Literal::Usize(2)),
                    Blad::Literal(Literal::Usize(6)),
                ]),
                Blad::List(vec![
                    Blad::Literal(Literal::Usize(3)),
                    Blad::Literal(Literal::Usize(3)),
                ]),
            ])
        );
    }
}
