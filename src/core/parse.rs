use super::{Blad, BladError, Keyword, Literal};

pub fn parse(input: &str) -> Result<Blad, BladError> {
    let tokens = tokenize(input);
    let mut program = vec![Blad::Keyword(Keyword::Do)];
    let mut index = 0;

    loop {
        let (ast, i) = parse_tokens(&tokens[index..tokens.len()])?;
        program.push(ast);

        index += i;
        if index == tokens.len() {
            break;
        }
    }

    Ok(Blad::List(program))
}

fn tokenize(input: &str) -> Vec<String> {
    input
        .replace("(", " ( ")
        .replace(")", " ) ")
        .replace("'", " ' ")
        .split_whitespace()
        .map(str::to_string)
        .collect()
}

fn parse_tokens<S: AsRef<str>>(tokens: &[S]) -> Result<(Blad, usize), BladError> {
    match tokens[0].as_ref() {
        "(" => {
            let mut blaadjes = vec![];
            // Start after the opening `(`
            let mut index = 1;

            while tokens[index].as_ref() != ")" {
                let (blad, steps) = parse_tokens(&tokens[index..tokens.len()])?;
                blaadjes.push(blad);
                index += steps;
            }

            // Skip the closing `)`
            index += 1;

            if blaadjes.is_empty() {
                Ok((Blad::Unit, index))
            } else {
                Ok((Blad::List(blaadjes), index))
            }
        }
        ")" => Err(BladError::UnexpectedToken(")".into())),
        "'" => {
            let (blad, steps) = parse_tokens(&tokens[1..tokens.len()])?;
            Ok((Blad::Quote(Box::new(blad)), steps + 1))
        }
        t => Ok((parse_token(&t)?, 1)),
    }
}

fn parse_token(token: &str) -> Result<Blad, BladError> {
    // Numbers
    if token.chars().next().unwrap().is_numeric() {
        return parse_token_numeric(token);
    }

    // Keywords / Symbols
    match token {
        "+" => Ok(Blad::Keyword(Keyword::Add)),
        "-" => Ok(Blad::Keyword(Keyword::Subtract)),
        "<" => Ok(Blad::Keyword(Keyword::LessThan)),
        "=" => Ok(Blad::Keyword(Keyword::Equal)),
        ">" => Ok(Blad::Keyword(Keyword::GreaterThan)),
        "append" => Ok(Blad::Keyword(Keyword::Append)),
        "cons" => Ok(Blad::Keyword(Keyword::Cons)),
        "do" => Ok(Blad::Keyword(Keyword::Do)),
        "fn" => Ok(Blad::Keyword(Keyword::Lambda)),
        "head" => Ok(Blad::Keyword(Keyword::Head)),
        "if" => Ok(Blad::Keyword(Keyword::If)),
        "let" => Ok(Blad::Keyword(Keyword::Let)),
        "list" => Ok(Blad::Keyword(Keyword::List)),
        "macro" => Ok(Blad::Keyword(Keyword::Macro)),
        "tail" => Ok(Blad::Keyword(Keyword::Tail)),
        _ => Ok(Blad::Symbol(token.to_owned())),
    }
}

fn parse_token_numeric(token: &str) -> Result<Blad, BladError> {
    // Assume float
    if token.contains('.') {
        let float: f32 = token
            .parse()
            .map_err(|_| BladError::UnsupportedNumericType(token.into()))?;

        Ok(Blad::Literal(Literal::F32(float)))
    } else {
        let int: usize = token
            .parse()
            .map_err(|_| BladError::UnsupportedNumericType(token.into()))?;

        Ok(Blad::Literal(Literal::Usize(int)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_input() {
        assert_eq!(
            tokenize("(+ 1 (- 3 4))"),
            ["(", "+", "1", "(", "-", "3", "4", ")", ")"]
        );

        assert_eq!(
            tokenize("('y '(+ 1 2))"),
            ["(", "'", "y", "'", "(", "+", "1", "2", ")", ")"]
        );
    }

    #[test]
    fn parse() {
        let input = "(+ 1 4)";
        let tokens = tokenize(input);
        let (ast, _) = parse_tokens(&tokens).unwrap();

        assert_eq!(
            ast,
            Blad::List(vec![
                Blad::Keyword(Keyword::Add),
                Blad::Literal(Literal::Usize(1)),
                Blad::Literal(Literal::Usize(4)),
            ]),
        );
    }

    #[test]
    fn parse_quote() {
        let input = "(let x '(+ 2 3))";
        let tokens = tokenize(input);
        let (ast, _) = parse_tokens(&tokens).unwrap();

        assert_eq!(
            ast,
            Blad::List(vec![
                Blad::Keyword(Keyword::Let),
                Blad::Symbol("x".into()),
                Blad::Quote(Box::new(Blad::List(vec![
                    Blad::Keyword(Keyword::Add),
                    Blad::Literal(Literal::Usize(2)),
                    Blad::Literal(Literal::Usize(3)),
                ]))),
            ]),
        );
    }

    #[test]
    fn parse_quoted_symbol() {
        let input = "('x)";
        let tokens = tokenize(input);
        let (ast, _) = parse_tokens(&tokens).unwrap();

        assert_eq!(
            ast,
            Blad::List(vec![Blad::Quote(Box::new(Blad::Symbol("x".into())))]),
        );
    }
}
