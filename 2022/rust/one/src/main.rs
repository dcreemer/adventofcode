// solution to
// https://adventofcode.com/2022/day/1

use itertools::Itertools;  // for "sorted" -- not strictly needed, but it reads nicer
use std::fs;

fn parse(data: &str) -> Vec<i64> {
    // parse the data into groups of lines
    data.split("\n\n")
        // parse the group into lines of integers and sum them
        .map(|group| {
            group
                .lines()
                .map(|line| line.parse::<i64>().unwrap())
                .sum::<i64>()
        })
        // sort the Vec from highest to lowest
        .sorted_by(|a, b| b.cmp(a))
        // returning a Vec of integers
        .collect()
}

fn main() {
    let data = fs::read_to_string("input.txt").expect("err reading the file");
    let food = parse(&data);
    println!("Part 1: {}", food[0]);
    println!("Part 2: {}", food[0..3].iter().sum::<i64>());
}

#[cfg(test)]
mod test {
    use crate::parse;
    
    static DATA: &'static str = include_str!("test_data.txt");

    #[test]
    fn test_parse() {
        let food = parse(DATA);
        assert_eq!(food.len(), 5);
    }

    #[test]
    fn test_function() {
        let food = parse(DATA);
        assert_eq!(food[0], 24000);
        assert_eq!(food[0..3].iter().sum::<i64>(), 45000);
    }
}
