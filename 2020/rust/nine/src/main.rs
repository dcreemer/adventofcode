// solution to
// https://adventofcode.com/2020/day/9

use itertools::Itertools;
use std::fs;

fn parse(data: &str) -> Vec<i64> {
    data.lines().map(|l| l.parse::<i64>().unwrap()).collect()
}

fn is_sum_of_prev(data: &[i64], index: usize, prev_depth: usize) -> bool {
    let target = data[index];
    let range: &[i64] = &data[(index - prev_depth)..index];
    range
        .iter()
        .combinations(2)
        .any(|v| v[0] + v[1] == target)
}

fn find_part1(data: &[i64], prev_depth: usize) -> Option<i64> {
    for i in prev_depth .. data.len() {
        if !is_sum_of_prev(data, i, prev_depth) {
            return Some(data[i]);
        }
    }
    None
}

fn find_k_sum(data: &[i64], k: usize, target: i64) -> Option<&[i64]> {
    for i in 0..=(data.len()-k) {
        let r = &data[i..i+k];
        // eprintln!("t={}, k={}, r={:?}", target, k, r);
        if r.iter().sum::<i64>() == target {
            return Some(r)
        }
    }
    None
}

fn find_part2(data: &[i64], target: i64) -> Option<i64> {
    for l in 2..data.len() {
        if let Some(r) = find_k_sum(data, l, target) {
            return Some(r.iter().max().unwrap() + r.iter().min().unwrap())
        }
    }
    None
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("err reading the file");
    let v = parse(&contents);
    if let Some(p1) = find_part1(&v, 25) {
        println!("part 1 = {:?}", p1);
        println!("part 2 = {:?}", find_part2(&v, p1));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn test_part1() {
        let v = parse(DATA);
        assert_eq!(v[0], 35);
        assert_eq!(find_part1(&v, 5), Some(127));
    }

    #[test]
    fn test_part2() {
        let v = parse(DATA);
        let p1 = find_part1(&v, 5).unwrap();
        assert_eq!(find_part2(&v, p1), Some(62));
    }
}
