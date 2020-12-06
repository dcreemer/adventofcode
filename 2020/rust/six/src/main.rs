// solution to
// https://adventofcode.com/2020/day/4

use std::collections::HashSet;
use std::fs;
use std::iter::FromIterator;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("err reading the file");
    let anyone: usize = contents
        // break into groups
        .split("\n\n")
        // for each group, build a set of all chars and calc len
        .map(|s| HashSet::<char>::from_iter(s.chars().filter(|&c| c != '\n')).len())
        .sum();
    println!("part 1 = {}", anyone);
    let everyone: usize = contents
        .split("\n\n")
        .map(|s| {
            // compute the intersection off all members in a group. Each member
            // is a set of letters. Return the length of the intersection.
            let members: Vec<HashSet<char>> = s
                .split('\n')
                .map(|s| HashSet::from_iter(s.chars()))
                .collect();
            members
                .iter()
                .skip(1)
                .fold(members[0].clone(), |acc, hs| {
                    acc.intersection(&hs).cloned().collect()
                })
                .len()
        })
        .sum();
    println!("part 2 = {}", everyone);
}
