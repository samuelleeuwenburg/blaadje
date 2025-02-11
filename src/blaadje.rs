// https://norvig.com/lispy.html
#![allow(dead_code)]

use std::ops::{Add, Div, Mul, Sub};

pub fn parse(input: &str) -> Blad {
    let tokens = tokenize(input);

    let (ast, _) = parse_tokens(&tokens);
    ast
}

fn tokenize(input: &str) -> Vec<String> {
    input
        .replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(str::to_string)
        .collect()
}

fn parse_tokens<S: AsRef<str>>(tokens: &[S]) -> (Blad, usize) {
    match tokens[0].as_ref() {
        "(" => {
            let mut blaadjes = vec![];
            let mut index = 0;

            while tokens[index + 1].as_ref() != ")" {
                let (blad, steps) = parse_tokens(&tokens[index + 1..tokens.len()]);
                blaadjes.push(blad);
                index += steps;
            }

            index += 1;

            (Blad::List(blaadjes), index)
        }
        ")" => {
            panic!("unexpected )");
        }
        // @TODO: parse strings, literals, :atoms, keywords? etc. etc.
        atom => (parse_token(&atom), 1),
    }
}

fn parse_token(token: &str) -> Blad {
    // Check numbers first
    if token.chars().next().unwrap().is_numeric() {
        // Assume float
        if token.contains('.') {
            let float: f32 = token.parse().unwrap();
            Blad::F32(float)
        } else {
            let int: usize = token.parse().unwrap();
            Blad::Usize(int)
        }
    } else {
        // Match stringy content
        match token {
            "+" => Blad::Operator(Operator::Plus),
            "-" => Blad::Operator(Operator::Minus),
            "*" => Blad::Operator(Operator::Multiply),
            "/" => Blad::Operator(Operator::Divide),
            t => Blad::Atom(t.to_owned()),
        }
    }
}

pub fn eval(program: &Blad) -> Blad {
    match program {
        Blad::List(list) => {
            let op = list.get(0).unwrap();

            match op {
                Blad::List(_) => panic!("Expected keyword, atom or function call."),
                Blad::Operator(operator) => {
                    // Assumption: operators always work on 2 items
                    let a = eval(list.get(1).unwrap());
                    let b = eval(list.get(2).unwrap());

                    match (a, b) {
                        (Blad::Usize(x), Blad::Usize(y)) => {
                            Blad::Usize(eval_operator(operator, x, y))
                        }
                        (Blad::F32(x), Blad::F32(y)) => Blad::F32(eval_operator(operator, x, y)),
                        _ => panic!("Mismatched types for arithmetic"),
                    }
                }
                n => n.clone(),
            }
        }
        n => n.clone(),
    }
}

fn eval_operator<T>(operator: &Operator, a: T, b: T) -> T
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy,
{
    match operator {
        Operator::Plus => a + b,
        Operator::Minus => a - b,
        Operator::Multiply => a * b,
        Operator::Divide => a / b,
    }
}

#[derive(Debug, Clone)]
pub enum Blad {
    Atom(String),
    Operator(Operator),
    Usize(usize),
    F32(f32),
    List(Vec<Blad>),
}

#[derive(Debug, Clone, Copy)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let input = "(+ 1 (- 3 4))";
        let tokens = tokenize(input);

        assert_eq!(tokens, ["(", "+", "1", "(", "-", "3", "4", ")", ")"])
    }

    #[test]
    fn test_parse_tokens() {
        let input = "(+ 1 (list 3 4))";
        let tokens = tokenize(input);
        let (ast, _) = parse_tokens(&tokens);

        // @TODO: CHECKS!
    }
}
