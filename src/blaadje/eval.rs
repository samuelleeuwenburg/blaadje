use super::prelude::prelude_environment;
use super::{Blad, BladError, Environment, Keyword, Literal};

pub fn run_program(program: &[Blad]) -> Result<Blad, BladError> {
    let mut env = prelude_environment();

    println!("=======================================");
    println!("end of prelude run program:");

    for i in 0..program.len() - 1 {
        eval(&program[i], &mut env)?;
    }

    eval(program.last().unwrap(), &mut env)
}

pub fn eval(program: &Blad, env: &mut Environment) -> Result<Blad, BladError> {
    println!("eval> {:?}", program);

    match program {
        Blad::Unit => Ok(Blad::Unit),
        Blad::Literal(value) => Ok(Blad::Literal(value.clone())),
        Blad::Symbol(string) => match string.as_str() {
            "+" => Ok(Blad::Keyword(Keyword::Add)),
            "-" => Ok(Blad::Keyword(Keyword::Subtract)),
            "<" => Ok(Blad::Keyword(Keyword::LessThan)),
            "=" => Ok(Blad::Keyword(Keyword::Equal)),
            ">" => Ok(Blad::Keyword(Keyword::GreaterThan)),
            "do" => Ok(Blad::Keyword(Keyword::Do)),
            "head" => Ok(Blad::Keyword(Keyword::Head)),
            "if" => Ok(Blad::Keyword(Keyword::If)),
            "lambda" => Ok(Blad::Keyword(Keyword::Lambda)),
            "let" => Ok(Blad::Keyword(Keyword::Let)),
            "list" => Ok(Blad::Keyword(Keyword::List)),
            "tail" => Ok(Blad::Keyword(Keyword::Tail)),
            _ => env.get(string).cloned().ok_or(BladError::UndefinedSymbol),
        },
        Blad::List(list) => {
            if list.is_empty() {
                Ok(Blad::Unit)
            } else {
                process_operator(list, env)
            }
        }
        Blad::Quote(blad) => Ok(*blad.clone()),
        Blad::Lambda(closure, params, body) => {
            Ok(Blad::Lambda(closure.clone(), params.clone(), body.clone()))
        }
        Blad::Keyword(string) => Ok(Blad::Keyword(string.clone())),
    }
}

pub fn process_operator(list: &[Blad], env: &mut Environment) -> Result<Blad, BladError> {
    let operator = eval(list.get(0).unwrap(), env)?;

    println!("    > process_operator> {:?} {:?}", operator, list);

    match operator {
        Blad::Keyword(Keyword::Let) => match (&list[1], &list[2]) {
            (Blad::Symbol(key), value) => {
                let mut e = Environment::child_from(env);
                let result = eval(value, &mut e)?;
                println!("set:: {} -> {:?}", key, result);
                env.set(key, result)?;
                Ok(Blad::List(vec![]))
            }
            _ => Err(BladError::ExpectedSymbol),
        },

        Blad::Keyword(Keyword::List) => {
            let mut result = vec![];

            for i in 1..list.len() {
                let r = eval(&list[i], env)?;
                result.push(r);
            }

            Ok(Blad::List(result))
        }

        Blad::Keyword(Keyword::Do) => {
            for i in 1..list.len() - 1 {
                eval(&list[i], env)?;
            }

            eval(list.last().unwrap(), env)
        }

        Blad::Keyword(Keyword::Add) => {
            let mut result = vec![];

            for i in 0..list.len() - 1 {
                let r = eval(&list[i + 1], env)?;
                result.push(r);
            }

            match result[0] {
                Blad::Literal(Literal::Usize(_)) => {
                    let mut sum = 0;

                    for num in result.into_iter() {
                        if let Blad::Literal(Literal::Usize(x)) = num {
                            sum += x;
                        } else {
                            return Err(BladError::ExpectedUsize);
                        }
                    }

                    Ok(Blad::Literal(Literal::Usize(sum)))
                }
                Blad::Literal(Literal::F32(_)) => {
                    let mut sum = 0.0;

                    for num in result.into_iter() {
                        if let Blad::Literal(Literal::F32(x)) = num {
                            sum += x;
                        } else {
                            return Err(BladError::ExpectedF32);
                        }
                    }

                    Ok(Blad::Literal(Literal::F32(sum)))
                }
                _ => Err(BladError::ExpectedNumber),
            }
        }

        Blad::Keyword(Keyword::Subtract) => {
            let mut result = vec![];

            for i in 0..list.len() - 1 {
                let r = eval(&list[i + 1], env)?;
                result.push(r);
            }

            match result[0] {
                Blad::Literal(Literal::Usize(x)) => {
                    let mut sum = x;

                    for i in 1..result.len() - 1 {
                        if let Blad::Literal(Literal::Usize(x)) = result[i] {
                            sum -= x;
                        } else {
                            return Err(BladError::ExpectedUsize);
                        }
                    }

                    Ok(Blad::Literal(Literal::Usize(sum)))
                }
                Blad::Literal(Literal::F32(x)) => {
                    let mut sum = x;

                    for i in 1..result.len() - 1 {
                        if let Blad::Literal(Literal::F32(x)) = result[i] {
                            sum -= x;
                        } else {
                            return Err(BladError::ExpectedF32);
                        }
                    }

                    Ok(Blad::Literal(Literal::F32(sum)))
                }
                _ => Err(BladError::ExpectedNumber),
            }
        }

        Blad::Keyword(Keyword::Lambda) => {
            let params = &list[1];
            let body = &list[2];

            match (params, list.len()) {
                (Blad::List(p), 3) => {
                    let mut param_strings = vec![];

                    for blad in p.into_iter() {
                        if let Blad::Symbol(param) = blad {
                            param_strings.push(param.clone());
                        } else {
                            return Err(BladError::ExpectedSymbol);
                        }
                    }

                    Ok(Blad::Lambda(
                        env.clone(),
                        param_strings,
                        Box::new(body.clone()),
                    ))
                }
                (_, 3) => Err(BladError::ExpectedList),
                _ => Err(BladError::IncorrectLambdaSyntax),
            }
        }

        Blad::Lambda(closure, params, body) => {
            if params.len() != list.len() - 1 {
                return Err(BladError::WrongNumberOfArguments);
            }

            let mut e = closure.clone();
            e.set_parent(&env);

            for (i, p) in params.iter().enumerate() {
                e.set(p, eval(&list[i + 1], env)?)?;
            }

            eval(&body, &mut e)
        }

        Blad::Keyword(Keyword::Head) => match eval(&list[1], env)? {
            Blad::Unit => Ok(Blad::Unit),
            Blad::List(l) => Ok(l[0].clone()),
            _ => Err(BladError::ExpectedList),
        },

        Blad::Keyword(Keyword::Tail) => match eval(&list[1], env)? {
            Blad::List(l) => {
                let mut r = vec![];
                r.extend_from_slice(&l[1..l.len()]);
                Ok(Blad::List(r))
            }
            _ => Err(BladError::ExpectedList),
        },

        Blad::Keyword(Keyword::If) => match eval(&list[1], env)? {
            Blad::Unit => eval(&list[3], env),
            Blad::Literal(Literal::Usize(x)) => {
                if x != 0 {
                    eval(&list[2], env)
                } else {
                    eval(&list[3], env)
                }
            }
            _ => eval(&list[2], env),
        },

        Blad::Keyword(Keyword::Equal) => match (eval(&list[1], env)?, eval(&list[2], env)?) {
            (Blad::Literal(Literal::Usize(x)), Blad::Literal(Literal::Usize(y))) => {
                let output = if x == y { 1 } else { 0 };

                Ok(Blad::Literal(Literal::Usize(output)))
            }
            _ => Err(BladError::ExpectedSameTypes),
        },

        Blad::Keyword(Keyword::GreaterThan) => match (eval(&list[1], env)?, eval(&list[2], env)?) {
            (Blad::Literal(Literal::Usize(x)), Blad::Literal(Literal::Usize(y))) => {
                let output = if x > y { 1 } else { 0 };

                Ok(Blad::Literal(Literal::Usize(output)))
            }
            _ => Err(BladError::ExpectedSameTypes),
        },

        Blad::Keyword(Keyword::LessThan) => match (eval(&list[1], env)?, eval(&list[2], env)?) {
            (Blad::Literal(Literal::Usize(x)), Blad::Literal(Literal::Usize(y))) => {
                let output = if x < y { 1 } else { 0 };

                Ok(Blad::Literal(Literal::Usize(output)))
            }
            _ => Err(BladError::ExpectedSameTypes),
        },

        _ => Err(BladError::ExpectedProcedure),
    }
}

#[cfg(test)]
mod tests {
    use super::super::parse;
    use super::*;

    #[test]
    fn test_variables_should_resolve() {
        let program = parse("(let x 5) x").unwrap();

        assert_eq!(
            run_program(&program).unwrap(),
            Blad::Literal(Literal::Usize(5))
        );
    }

    #[test]
    fn test_unknown_variables_should_crash() {
        let program = parse("x").unwrap();

        assert_eq!(
            run_program(&program).unwrap_err(),
            BladError::UndefinedSymbol
        );
    }

    #[test]
    fn test_quoted_variables_should_not_eval() {
        let program = parse("(let x '(+ 3 2)) x").unwrap();

        assert_eq!(
            run_program(&program).unwrap(),
            Blad::List(vec![
                Blad::Symbol("+".into()),
                Blad::Literal(Literal::Usize(3)),
                Blad::Literal(Literal::Usize(2)),
            ]),
        );
    }

    #[test]
    fn test_variables_should_eval_before_set() {
        let program = parse("(let x (+ 1 2)) x").unwrap();

        assert_eq!(
            run_program(&program).unwrap(),
            Blad::Literal(Literal::Usize(3))
        );
    }

    #[test]
    fn test_addition_should_compute() {
        let program = parse("(+ 1 2)").unwrap();

        assert_eq!(
            run_program(&program).unwrap(),
            Blad::Literal(Literal::Usize(3))
        );
    }

    #[test]
    fn test_before_applying_a_function_subexpressions_must_be_evaluated() {
        let program = parse("(+ (+ 1 2) (+ 3 4))").unwrap();

        assert_eq!(
            run_program(&program).unwrap(),
            Blad::Literal(Literal::Usize(10))
        );
    }

    #[test]
    fn test_quoted_data_should_not_evaluate() {
        let program = parse("'(x y z)").unwrap();

        assert_eq!(
            run_program(&program).unwrap(),
            Blad::List(vec![
                Blad::Symbol("x".into()),
                Blad::Symbol("y".into()),
                Blad::Symbol("z".into()),
            ]),
        );
    }

    #[test]
    fn test_lambda_identity() {
        let program = parse(
            "
            (let identity (lambda (x) x))
            (identity 10)
        ",
        )
        .unwrap();

        assert_eq!(
            run_program(&program).unwrap(),
            Blad::Literal(Literal::Usize(10)),
        );
    }

    #[test]
    fn test_lambda_recursion() {
        let program = parse(
            "
            (let r (lambda (x y)
                (if (>= x 10)
                    y
                    (r (+ x 1) (+ y 1))
                )))

            (r 3 0)
        ",
        )
        .unwrap();

        assert_eq!(
            run_program(&program).unwrap(),
            Blad::Literal(Literal::Usize(7)),
        );
    }

    #[test]
    fn test_lambda_currying() {
        let program = parse(
            "
            (let adder (lambda (x) (lambda (y) (+ x y))))
            (let add_five (adder 5))
            (add_five 2)
        ",
        )
        .unwrap();

        assert_eq!(
            run_program(&program).unwrap(),
            Blad::Literal(Literal::Usize(7)),
        );
    }

    #[test]
    fn test_lambda_multiple_arguments() {
        let program = parse(
            "
            (let summer
                (lambda (a b c d) (+ a b c d)))

            (summer 1 2 3 4)
        ",
        )
        .unwrap();

        assert_eq!(
            run_program(&program).unwrap(),
            Blad::Literal(Literal::Usize(10)),
        );
    }

    #[test]
    fn test_head() {
        let program = parse(
            "
            (head '(1 2 3 4))
        ",
        )
        .unwrap();

        assert_eq!(
            run_program(&program).unwrap(),
            Blad::Literal(Literal::Usize(1)),
        );
    }

    #[test]
    fn test_head_empty() {
        let program = parse(
            "
            (head '())
        ",
        )
        .unwrap();

        assert_eq!(run_program(&program).unwrap(), Blad::Unit);
    }

    #[test]
    fn test_tail() {
        let program = parse(
            "
            (tail '(1 2 3 4))
        ",
        )
        .unwrap();

        assert_eq!(
            run_program(&program).unwrap(),
            Blad::List(vec![
                Blad::Literal(Literal::Usize(2)),
                Blad::Literal(Literal::Usize(3)),
                Blad::Literal(Literal::Usize(4)),
            ])
        );
    }

    #[test]
    fn test_list() {
        let program = parse(
            "
            (list
                (+ 10 1)
                (+ 10 2)
                (+ 10 3)
                (+ 10 4)
            )
        ",
        )
        .unwrap();

        assert_eq!(
            run_program(&program).unwrap(),
            Blad::List(vec![
                Blad::Literal(Literal::Usize(11)),
                Blad::Literal(Literal::Usize(12)),
                Blad::Literal(Literal::Usize(13)),
                Blad::Literal(Literal::Usize(14)),
            ])
        );
    }

    #[test]
    fn test_do() {
        let program = parse(
            "
            (do
                (let x (+ 1 2))
                (let y (+ x 3))
                (let z (+ x y))
                (+ x y z))
        ",
        )
        .unwrap();

        assert_eq!(
            run_program(&program).unwrap(),
            Blad::Literal(Literal::Usize(18)),
        );
    }

    #[test]
    fn test_if_true() {
        let program = parse("(if 1 2 3)").unwrap();

        assert_eq!(
            run_program(&program).unwrap(),
            Blad::Literal(Literal::Usize(2)),
        );
    }

    #[test]
    fn test_if_false() {
        let program = parse("(if 0 2 3)").unwrap();

        assert_eq!(
            run_program(&program).unwrap(),
            Blad::Literal(Literal::Usize(3)),
        );
    }
}
