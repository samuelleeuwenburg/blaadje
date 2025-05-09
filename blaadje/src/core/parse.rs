use super::{Blad, Error, Keyword, Literal};

pub fn parse(input: &str) -> Result<Vec<Blad>, Error> {
    let tokens = tokenize(input);
    let mut nodes = vec![];
    let mut index = 0;

    loop {
        let (ast, i) = parse_tokens(&tokens[index..tokens.len()])?;
        nodes.push(ast);

        index += i;
        if index == tokens.len() {
            break;
        }
    }

    Ok(nodes)
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

fn parse_tokens(tokens: &[String]) -> Result<(Blad, usize), Error> {
    match tokens.get(0).map(|s| s.as_str()) {
        Some("(") => {
            let mut blaadjes = vec![];
            // Start after the opening `(`
            let mut index = 1;

            while tokens.get(index).ok_or(Error::ParseError(index))? != ")" {
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
        Some(")") => Err(Error::UnexpectedToken(")".into())),
        Some("'") => {
            let (blad, steps) = parse_tokens(&tokens[1..tokens.len()])?;
            Ok((Blad::Quote(Box::new(blad)), steps + 1))
        }
        Some(t) => Ok((parse_token(&t)?, 1)),
        None => Ok((Blad::Unit, 0)),
    }
}

fn parse_token(token: &str) -> Result<Blad, Error> {
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
        "call" => Ok(Blad::Keyword(Keyword::Call)),
        "cast" => Ok(Blad::Keyword(Keyword::Cast)),
        "cons" => Ok(Blad::Keyword(Keyword::Cons)),
        "fn" => Ok(Blad::Keyword(Keyword::Lambda)),
        "head" => Ok(Blad::Keyword(Keyword::Head)),
        "if" => Ok(Blad::Keyword(Keyword::If)),
        "let" => Ok(Blad::Keyword(Keyword::Let)),
        "list" => Ok(Blad::Keyword(Keyword::List)),
        "macro" => Ok(Blad::Keyword(Keyword::Macro)),
        "samples" => Ok(Blad::Keyword(Keyword::Samples)),
        "string" => Ok(Blad::Keyword(Keyword::String)),
        "tail" => Ok(Blad::Keyword(Keyword::Tail)),
        s if s.starts_with(':') => Ok(Blad::Atom(token.to_owned())),
        s if s.starts_with('"') && s.ends_with('"') => {
            let mut string = s.to_owned();

            // Remove `"`
            string.pop();
            string.remove(0);

            Ok(Blad::Literal(Literal::String(string)))
        }
        _ => Ok(Blad::Symbol(token.to_owned())),
    }
}

fn parse_token_numeric(token: &str) -> Result<Blad, Error> {
    // Assume float
    if token.contains('.') {
        let float: f32 = token
            .parse()
            .map_err(|_| Error::UnsupportedNumericType(token.into()))?;

        Ok(Blad::Literal(Literal::F32(float)))
    } else {
        let int: usize = token
            .parse()
            .map_err(|_| Error::UnsupportedNumericType(token.into()))?;

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
