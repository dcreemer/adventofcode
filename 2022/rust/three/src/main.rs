// solution to
// https://adventofcode.com/2022/day/3

use itertools::izip;
use std::collections::HashSet;
use std::fs;

type Pack = HashSet<char>;

fn parse_line(line: &str) -> Pack {
    HashSet::from_iter(line.chars())
}

fn split_and_parse(line: &str) -> (Pack, Pack) {
    assert_eq!(line.len() % 2, 0);
    (
        HashSet::from_iter(line.chars().take(line.len() / 2)),
        HashSet::from_iter(line.chars().skip(line.len() / 2)),
    )
}

fn priority(c: char) -> i64 {
    assert!(c.is_ascii_alphabetic());
    let p_a = 'a' as i64 - 1;
    let p_cap_a = 'A' as i64 - 27;
    match c {
        'a'..='z' => c as i64 - p_a,
        'A'..='Z' => c as i64 - p_cap_a,
        _ => 0,
    }
}

fn part_1(data: &str) -> i64 {
    data.lines()
        // parse each line into two HashSets
        .map(|line| split_and_parse(line))
        // find the common character
        .map(|(p1, p2)| *p1.intersection(&p2).nth(0).unwrap())
        // extract the priorities
        .map(|c| priority(c))
        // and sum them
        .sum()
}

// intersect multiple sets. Adapted from:
// https://github.com/rust-lang/rfcs/issues/2023
fn intersect_all(packs: &[Pack]) -> Pack {
    packs
    .iter()
    .skip(1)
    // repeatedly intersect the first set with each subsequent item in the slice
    .fold(packs[0].clone(), |acc, hs| {
        acc.intersection(hs).cloned().collect()
    })
}

fn part_2(data: &str) -> i64 {
    // create three iterators, each offest by 1 and skipping three
    // this gives us [ABCDEFGHI] -> (ABC, DEF, GHI), ...
    izip!(
        data.lines().step_by(3),
        data.lines().skip(1).step_by(3),
        data.lines().skip(2).step_by(3)
    )
    // for each set of three lines:
    .map(|(line_1, line_2, line_3)| {
        // find the common intersection:
        let i = intersect_all( &[
            parse_line(line_1),
            parse_line(line_2),
            parse_line(line_3)
        ]);
        // and return the priority of the one and only item:
        assert_eq!(i.len(), 1);
        priority(*i.iter().nth(0).unwrap())
    })
    // return the sum of all priorities
    .sum()
}

fn main() {
    let data = fs::read_to_string("input.txt").expect("err reading the file");
    println!("part 1 = {}", part_1(&data));
    println!("part 2 = {}", part_2(&data));
}

#[cfg(test)]
mod test {
    use crate::*;

    static DATA: &'static str = include_str!("test_data.txt");

    #[test]
    fn test_parse_line() {
        let (a, b) = split_and_parse("vJrwpWtwJgWrhcsFMMfFFhFp");
        assert_eq!(a.len(), 8);
        assert_eq!(b.len(), 7);
    }

    #[test]
    fn test_priority() {
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('z'), 26);
        assert_eq!(priority('A'), 27);
        assert_eq!(priority('Z'), 52);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(DATA), 157);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(DATA), 70);
    }
}
