// solution to
// https://adventofcode.com/2020/day/18

use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::iter::Peekable;

#[derive(Debug, PartialEq)]
enum Token {
    Number(i64),
    Mult,
    Add,
    Open,
    Close,
}

// parse text into Tokens
fn parse(data: &str) -> Result<Vec<Token>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)|\+|\*|\(|\)").unwrap();
    }
    let result = RE
        .find_iter(data)
        .map(|m| match m.as_str() {
            "(" => Ok(Token::Open),
            ")" => Ok(Token::Close),
            "+" => Ok(Token::Add),
            "*" => Ok(Token::Mult),
            s => s.parse::<i64>().map(|v| Token::Number(v)),
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(result)
}

// part 1

// EXPR1 = TERM1 ('+' | '*') TERM1 [('+' | '*') TERM1 ...]
fn eval_expr1(mut tokens: &mut Peekable<std::slice::Iter<Token>>) -> Result<i64> {
    let mut acc = eval_term1(&mut tokens)?;
    loop {
        match tokens.peek() {
            Some(Token::Add) => {
                let _ = tokens.next();
                let rhs = eval_term1(&mut tokens)?;
                acc += rhs;
            }
            Some(Token::Mult) => {
                let _ = tokens.next();
                let rhs = eval_term1(&mut tokens)?;
                acc *= rhs;
            }
            _ => {
                return Ok(acc);
            }
        }
    }
}

// TERM1 = NUM | '(' EXPR1 ')'
fn eval_term1(mut tokens: &mut Peekable<std::slice::Iter<Token>>) -> Result<i64> {
    match tokens.next() {
        Some(Token::Number(v)) => Ok(*v),
        Some(Token::Open) => {
            let v = eval_expr1(&mut tokens)?;
            if tokens.next() != Some(&Token::Close) {
                Err(anyhow!("Expected ')'"))
            } else {
                Ok(v)
            }
        }
        Some(t) => Err(anyhow!("unexpected token {:?}", t)),
        None => Err(anyhow!("unexpected end")),
    }
}

fn part_1(data: &str) -> i64 {
    if let Ok(v) = data
        .lines()
        .map(|line| {
            let parsed = parse(line)?;
            let mut p = parsed.iter().peekable();
            eval_expr1(&mut p)
        })
        .collect::<Result<Vec<_>>>()
    {
        v.iter().sum()
    } else {
        panic!("no");
    }
}

// part 2

// Sum = Product '+' Product
fn eval_sum(mut tokens: &mut Peekable<std::slice::Iter<Token>>) -> Result<i64> {
    let mut acc = eval_term2(&mut tokens)?;
    loop {
        if let Some(Token::Add) = tokens.peek() {
            let _ = tokens.next();
            let rhs = eval_term2(&mut tokens)?;
            acc += rhs;
        } else {
            return Ok(acc);
        }
    }
}

// Product = (Sum | Term) '*' Term
fn eval_product(mut tokens: &mut Peekable<std::slice::Iter<Token>>) -> Result<i64> {
    let mut acc = eval_sum(&mut tokens)?;
    loop {
        if let Some(Token::Mult) = tokens.peek() {
            let _ = tokens.next();
            let rhs = eval_sum(&mut tokens)?;
            acc *= rhs;
        } else {
            return Ok(acc);
        }
    }
}

// TERM2 = Num | '(' Product ')'
fn eval_term2(mut tokens: &mut Peekable<std::slice::Iter<Token>>) -> Result<i64> {
    match tokens.next() {
        Some(Token::Number(v)) => Ok(*v),
        Some(Token::Open) => {
            let v = eval_product(&mut tokens)?;
            if tokens.next() != Some(&Token::Close) {
                Err(anyhow!("Expected ')'"))
            } else {
                Ok(v)
            }
        }
        Some(t) => Err(anyhow!("unexpected token {:?}", t)),
        None => Err(anyhow!("unexpected end")),
    }
}

fn part_2(data: &str) -> i64 {
    if let Ok(v) = data
        .lines()
        .map(|line| {
            let parsed = parse(line)?;
            let mut p = parsed.iter().peekable();
            eval_product(&mut p)
        })
        .collect::<Result<Vec<_>>>()
    {
        v.iter().sum()
    } else {
        panic!("no");
    }
}

fn main() -> Result<()> {
    let contents = include_str!("../input.txt");
    println!("part 1 = {}", part_1(&contents));
    println!("part 2 = {}", part_2(&contents));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part_1("1 + 2 * 3 + 4 * 5 + 6"), 71);
        assert_eq!(part_1("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(part_1("2 * 3 + (4 * 5)"), 26);
        assert_eq!(part_1("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
        assert_eq!(part_1("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
        assert_eq!(
            part_1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            13632
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2("1 + 2 * 3 + 4 * 5 + 6"), 231);
        assert_eq!(part_2("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(part_2("2 * 3 + (4 * 5)"), 46);
        assert_eq!(part_2("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
        assert_eq!(part_2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 669060);
        assert_eq!(
            part_2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            23340
        );
    }
}
