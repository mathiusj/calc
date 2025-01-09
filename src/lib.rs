#[derive(Debug)]
enum Expr {
    Number(i64),
    Add(Box<Expr>, Box<Expr>)
}


#[derive(Debug)]
enum EvalError {

}

fn eval(expr: &Expr) -> Result<i64, EvalError> {
    match expr {
        Expr::Number(x) => Ok(*x),
        Expr::Add(left, right) => Ok(eval(left)? + eval(right)?),
    }
}

#[derive(Debug)]
enum ParseError {
    NotANumber
}

fn parse(input: &str) -> Result<Expr, ParseError> {
    let mut stack: Vec<Expr> = Vec::new();
    for word in input.split_ascii_whitespace() {
        match word {
            "+" => {
                if let Some(left) = stack.pop() {
                    if let Some(right) = stack.pop() {
                        stack.push(Expr::Add(Box::new(left), Box::new(right)))
                    }
                }
                // Add error handling?
            }
            _ => {
                let x = word.parse::<i64>().map_err(|_| ParseError::NotANumber)?;
                stack.push(Expr::Number(x))
            }
        }
    };
    // assert_eq!(stack.len(), 1);
    let res = stack.pop().unwrap();
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_test() {
        let input = "7";
        let expr = parse(input).unwrap();
        let value = eval(&expr).unwrap();
        println!("{expr:?}");
        println!("{value}");
        assert_eq!(value, 7);
    }

    #[test]
    fn add_test() {
        let input = "3 5 +";
        let expr = parse(input).unwrap();
        let value = eval(&expr).unwrap();
        assert_eq!(value, 8);
    }

    #[ignore]
    #[test]
    fn smoke_test() {
        let input = "3 sqr 4 sqr + 5 sqr -";
        let expr = parse(input).unwrap();
        let value = eval(&expr).unwrap();
        assert_eq!(value, 0);
    }
}
