use std::fs;
use std::path::Path;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Token {
    Number(i64),
    Add,
    Mul,
    PStart,
    PEnd,
}

impl Token {
    fn is_number(&self) -> bool {
        match self {
            Self::Number(_) => true,
            _ => false,
        }
    }

    fn value(&self) -> i64 {
        match self {
            Self::Number(v) => *v,
            _ => panic!("No value on non-number token"),
        }
    }
}

fn parse(filename: &Path) -> Result<Vec<Vec<Token>>, String> {
    fs::read_to_string(filename)
        .map_err(|err| format!("Failed to read data for day 18: {}", err))?
        .lines()
        .map(|line| {
            let mut nesting = 0;
            let mut current: Option<Token> = None;
            let mut tokens = Vec::new();
            for c in line.chars() {
                match c {
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                        if current.is_some() {
                            current = Some(Token::Number(current.unwrap().value() * 10 + (c as i64) - 48));
                        } else {
                            current = Some(Token::Number((c as i64) - 48));
                        }
                    }
                    '+' => {
                        if let Some(token) = current {
                            tokens.push(token);
                        }
                        current = None;
                        tokens.push(Token::Add);
                    }
                    '*' => {
                        if let Some(token) = current {
                            tokens.push(token);
                        }
                        current = None;
                        tokens.push(Token::Mul);
                    }
                    '(' => {
                        if let Some(token) = current {
                            tokens.push(token);
                        }
                        current = None;
                        tokens.push(Token::PStart);
                        nesting += 1;
                    }
                    ')' => {
                        if nesting > 0 {
                            if let Some(token) = current {
                                tokens.push(token);
                            }
                            current = None;
                            tokens.push(Token::PEnd);
                            nesting -= 1;
                        } else {
                            Err("Mismatched closing parenthesis".to_string())?
                        }
                    }
                    ' ' => {
                        if let Some(token) = current {
                            tokens.push(token);
                        }
                        current = None;
                    }
                    c => Err(format!("Unknown symbol: {}", c))?,
                }
            }

            if let Some(token) = current {
                tokens.push(token);
            }

            if nesting > 0 {
                Err("Unbalanced parentheses".to_string())?
            }

            Ok(tokens)
        })
        .collect()
}

fn shunting_yard<F>(expr: &[Token], prec: F) -> Result<i64, String>
where
    F: Fn(&Token) -> u8,
{
    let mut ops = Vec::new();
    let mut queue = Vec::new();

    // shunting yard main to build output queue
    for token in expr {
        match token {
            Token::Number(_) => queue.push(*token),
            Token::Add | Token::Mul => {
                while !ops.is_empty() {
                    let last = ops.last().unwrap();
                    if last == &Token::PStart {
                        break;
                    } else if prec(last) >= prec(token) {
                        let rhs = queue.pop().unwrap();
                        let lhs = queue.pop().unwrap();
                        match ops.pop().unwrap() {
                            Token::Add => queue.push(Token::Number(rhs.value() + lhs.value())),
                            Token::Mul => queue.push(Token::Number(rhs.value() * lhs.value())),
                            _ => panic!("Malformed output queue"),
                        }
                    } else {
                        break;
                    }
                }
                ops.push(*token);
            }
            Token::PStart => ops.push(*token),
            Token::PEnd => {
                while !ops.is_empty() {
                    let last = ops.last().unwrap();
                    if last != &Token::PStart {
                        let rhs = queue.pop().unwrap();
                        let lhs = queue.pop().unwrap();
                        match ops.pop().unwrap() {
                            Token::Add => queue.push(Token::Number(rhs.value() + lhs.value())),
                            Token::Mul => queue.push(Token::Number(rhs.value() * lhs.value())),
                            _ => panic!("Malformed output queue"),
                        }
                    } else {
                        ops.pop();
                        break;
                    }
                }
            }
        }
    }

    // pop remaining operator to output queue
    while !ops.is_empty() {
        let rhs = queue.pop().unwrap();
        let lhs = queue.pop().unwrap();
        match ops.pop().unwrap() {
            Token::Add => queue.push(Token::Number(rhs.value() + lhs.value())),
            Token::Mul => queue.push(Token::Number(rhs.value() * lhs.value())),
            _ => panic!("Malformed output queue"),
        }
    }

    Ok(queue[0].value())
}

pub fn part01(filename: &Path) -> Result<String, String> {
    let mut expressions = parse(filename)?;

    Ok(format!(
        "Sum of all expressions: {}",
        expressions
            .iter()
            .map(|expr| shunting_yard(&expr, |token| {
                match token {
                    Token::Mul | Token::Add => 1,
                    _ => panic!("Not an operator: {:?}", token),
                }
            })
            .unwrap())
            .sum::<i64>()
    ))
}

pub fn part02(filename: &Path) -> Result<String, String> {
    let mut expressions = parse(filename)?;

    Ok(format!(
        "Sum of all expressions: {}",
        expressions
            .iter()
            .map(|expr| shunting_yard(&expr, |token| {
                match token {
                    Token::Mul => 1,
                    Token::Add => 2,
                    _ => panic!("Not an operator: {:?}", token),
                }
            })
            .unwrap())
            .sum::<i64>()
    ))
}
