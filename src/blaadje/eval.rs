use super::{Blad, BladError, Environment, Keyword, Literal};

pub fn run_program(program: &[Blad]) -> Result<Blad, BladError> {
    let mut env = Environment::new();

    for i in 0..program.len() - 1 {
        eval(&program[i], &mut env)?;
    }

    eval(program.last().unwrap(), &mut env)
}

pub fn eval(program: &Blad, env: &mut Environment) -> Result<Blad, BladError> {
    println!("eval> {:?}", program);

    match program {
        Blad::Literal(value) => Ok(Blad::Literal(value.clone())),
        Blad::Symbol(string) => match string.as_str() {
            "let" => Ok(Blad::Keyword(Keyword::Let)),
            "list" => Ok(Blad::Keyword(Keyword::List)),
            "lambda" => Ok(Blad::Keyword(Keyword::Lambda)),
            "+" => Ok(Blad::Keyword(Keyword::Add)),
            _ => env.get(string).cloned().ok_or(BladError::UndefinedSymbol),
        },
        Blad::List(list) => process_operator(list, env),
        Blad::Quote(blad) => Ok(*blad.clone()),
        Blad::Lambda(_closure, _params, _body) => Ok(Blad::Literal(Literal::Usize(0))),
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
                env.set(key, result)?;
                println!("set:: {:?}", env);
                Ok(Blad::List(vec![]))
            }
            _ => Err(BladError::ExpectedSymbol),
        },

        Blad::Keyword(Keyword::List) => {
            let mut result = vec![];

            for i in 0..list.len() - 1 {
                let r = eval(&list[i + 1], env)?;
                result.push(r);
            }

            Ok(Blad::List(result))
        }

        Blad::Keyword(Keyword::Add) => match (eval(&list[1], env)?, eval(&list[2], env)?) {
            (Blad::Literal(Literal::Usize(x)), Blad::Literal(Literal::Usize(y))) => {
                Ok(Blad::Literal(Literal::Usize(x + y)))
            }
            _ => Err(BladError::ExpectedSymbol),
        },

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

            let mut e = Environment::child_from(&closure);

            for (i, p) in params.iter().enumerate() {
                e.set(p, eval(&list[i + 1], env)?)?;
            }

            eval(&body, &mut e)
        }

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
    fn test_lambda_multiple_arguments() {
        let program = parse(
            "
            (let identity (lambda (x y) (+ x y)))
            (identity 10 2)
        ",
        )
        .unwrap();

        assert_eq!(
            run_program(&program).unwrap(),
            Blad::Literal(Literal::Usize(12)),
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
}
