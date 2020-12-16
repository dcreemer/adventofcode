// solution to
// https://adventofcode.com/2020/day/16

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;

type Ticket = Vec<i64>;

#[derive(Debug)]
struct Field {
    name: String,
    low1: i64,
    high1: i64,
    low2: i64,
    high2: i64,
}

impl Field {
    fn parse(data: &str) -> Field {
        lazy_static! {
            // <name>: 6-11 or 33-44
            static ref RE: Regex = Regex::new(r"^([a-z ]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
        }
        if let Some(cap) = RE.captures(data) {
            Field {
                name: cap.get(1).unwrap().as_str().to_string(),
                low1: cap.get(2).unwrap().as_str().parse::<i64>().unwrap(),
                high1: cap.get(3).unwrap().as_str().parse::<i64>().unwrap(),
                low2: cap.get(4).unwrap().as_str().parse::<i64>().unwrap(),
                high2: cap.get(5).unwrap().as_str().parse::<i64>().unwrap(),
            }
        } else {
            panic!("parse error!");
        }
    }

    fn is_valid(&self, i: i64) -> bool {
        i >= self.low1 && i <= self.high1 || i >= self.low2 && i <= self.high2
    }
}

fn part_1(data: &str) -> i64 {
    let sx: Vec<&str> = data.split("\n\n").collect();
    let rules: Vec<Field> = sx[0].lines().map(|l| Field::parse(l)).collect();
    let nearby_tickets: Vec<Ticket> = sx[2]
        .lines()
        .skip(1)
        .map(|l| l.split(',').map(|n| n.parse::<i64>().unwrap()).collect())
        .collect();
    let mut accum = 0;
    for t in &nearby_tickets {
        for v in t {
            if !rules.iter().any(|f| f.is_valid(*v)) {
                accum += *v;
            }
        }
    }
    accum
}

fn ticket_is_valid(t: &Ticket, rules: &[Field]) -> bool {
    for v in t {
        if !rules.iter().any(|f| f.is_valid(*v)) {
            return false;
        }
    }
    true
}

fn part_2(data: &str) -> i64 {
    let sx: Vec<&str> = data.split("\n\n").collect();
    let rules: Vec<Field> = sx[0].lines().map(|l| Field::parse(l)).collect();
    let my_ticket: Ticket = sx[1]
        .split('\n')
        .nth(1)
        .unwrap()
        .split(',')
        .map(|n| n.parse::<i64>().unwrap())
        .collect();
    let mut all_tickets: Vec<Ticket> = sx[2]
        .lines()
        .skip(1)
        .map(|l| l.split(',').map(|n| n.parse::<i64>().unwrap()).collect())
        .filter(|t| ticket_is_valid(t, &rules))
        .collect();
    let cols = my_ticket.len();
    all_tickets.push(my_ticket.clone());
    // truths maps fields to still true column indexes. All are true to begin with.
    let mut truths = HashMap::new();
    for f in &rules {
        let colset: HashSet<usize> = (0..cols).collect();
        truths.insert(&f.name, colset);
    }

    // for each field, find which column could not match this field
    for f in &rules {
        for i in 0..cols {
            if !all_tickets.iter().all(|t| f.is_valid(t[i])) {
                // remove column i from field f's truth table:
                let v = truths.get_mut(&f.name).unwrap();
                v.remove(&i);
            }
        }
    }

    // reduce the truth table to just the possible columns per field
    while !truths.values().all(|v| v.len() <= 1) {
        // now find all truths with just one column, and eliminate that column from others:
        let singles: Vec<usize> = truths
            .iter_mut()
            .filter(|(_, v)| v.len() == 1)
            .map(|(_, v)| *v.iter().next().unwrap())
            .collect();
        for s in &singles {
            for v in truths.values_mut() {
                if v.len() > 1 {
                    v.remove(s);
                }
            }
        }
    }
    truths
        .iter()
        .filter(|(&k, _)| k.starts_with("departure"))
        .map(|(_, v)| my_ticket[*v.iter().next().unwrap()])
        .product()
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("err reading the file");
    println!("part 1 = {:#?}", part_1(&contents));
    println!("part 2 = {:#?}", part_2(&contents));
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    #[test]
    fn test_part1() {
        assert_eq!(part_1(DATA), 71);
    }

    const DATA2: &str = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";

    #[test]
    fn test_part2() {
        assert_eq!(part_2(DATA2), 1);
    }
}
