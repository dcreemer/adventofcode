// solution to
// https://adventofcode.com/2020/day/18

use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

#[derive(Clone, Debug, PartialEq)]
enum Token {
    Number(i64),
    Mult,
    Add,
    Open,
    Close,
}

impl Copy for Token {}

fn parse(data: &str) -> Vec<Token> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)|\+|\*|\(|\)").unwrap();
    }
    let mut result = vec![];
    for mat in RE.find_iter(data) {
        match mat.as_str() {
            "(" => result.push(Token::Open),
            ")" => result.push(Token::Close),
            "+" => result.push(Token::Add),
            "*" => result.push(Token::Mult),
            s => {
                if let Ok(v) = s.parse::<i64>() {
                    result.push(Token::Number(v))
                } else {
                    panic!("bad token!");
                }
            }
        }
    }
    result
}

// destructively replace additions Num + Num with Num
// return true if anything is replaced
fn eval_adds(tokens: &mut Vec<Token>) -> bool {
    if let Some(idx) = tokens.iter().position(|x| *x == Token::Add) {
        if idx + 1 < tokens.len() {
            let mut r = false;
            let mut val = 0;
            if let (Token::Number(lhs), Token::Number(rhs)) = (&tokens[idx - 1], &tokens[idx + 1]) {
                r = true;
                val = lhs + rhs;
            }
            if r {
                tokens.remove(idx - 1);
                tokens.remove(idx - 1);
                tokens.remove(idx - 1);
                tokens.insert(idx - 1, Token::Number(val));
                return true;
            }
        } else {
            panic!("bad expr");
        }
    }
    return false;
}

// evaluate a string of Num .. op .. Num .. Ops left to right
fn eval_op_expr(tokens: &[Token], p2: bool) -> i64 {
    let mut tok2 = tokens.iter().map(|t| t.clone()).collect();
    let toks = if p2 {
        while eval_adds(&mut tok2) {}
        &tok2
    } else {
        tokens
    };
    let mut acc;
    let mut idx;
    if let Token::Number(v) = toks[0] {
        acc = v;
        idx = 1;
    } else {
        panic!("expr must start with number!");
    }
    while idx < toks.len() {
        if let (op, Token::Number(rhs)) = (&toks[idx], &toks[idx + 1]) {
            match op {
                Token::Add => acc += rhs,
                Token::Mult => acc *= rhs,
                _ => panic!("bad op"),
            }
            idx += 2;
        }
    }
    acc
}

// find the innermost parenthetical exression, and replace the contents
// with evaluation of it. Return true if one is found
fn eval_inner(tokens: &mut Vec<Token>, p2: bool) -> bool {
    let mut o = 0;
    let mut idx = 0;
    while idx < tokens.len() {
        if tokens[idx] == Token::Open {
            o = idx;
        } else if tokens[idx] == Token::Close {
            // replace o -> c
            let v = eval_op_expr(&tokens[o + 1..idx], p2);
            let mut i = 0;
            tokens.retain(|_| (i < o || i > idx, i += 1).0);
            tokens.insert(o, Token::Number(v));
            return true;
        }
        idx += 1;
    }
    false
}

fn eval_expr(data: &str, p2: bool) -> i64 {
    let mut tokens = parse(data);
    while eval_inner(&mut tokens, p2) {}
    eval_op_expr(&tokens, p2)
}

fn part_1(data: &str) -> i64 {
    data.lines().map(|l| eval_expr(l, false)).sum()
}

fn part_2(data: &str) -> i64 {
    data.lines().map(|l| eval_expr(l, true)).sum()
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("err reading the file");
    println!("part 1 = {}", part_1(&contents));
    println!("part 2 = {}", part_2(&contents));
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
