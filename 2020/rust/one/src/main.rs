// solution to
// https://adventofcode.com/2020/day/1

// Solution is brute-force 2- and 3-combinations. A more elegant solution
// would implement and general k-combinations Iterator, then filter the
// list for the first matching result.

use std::fs;

fn mult_of_2020sum_2(nums: &[i64]) -> Option<(i64, i64, i64)> {
    for (i, a) in nums.iter().enumerate() {
        for b in nums.iter().skip(i+1) {
            if (a + b) == 2020 {
                return Some((*a, *b, a * b));
            }
        }
    }
    None
}

fn mult_of_2020sum_3(nums: &[i64]) -> Option<(i64, i64, i64, i64)> {
    for (i, a) in nums.iter().enumerate() {
        for b in nums.iter().skip(i+1) {
            for c in nums.iter().skip(i+2) {
                if (a + b + c) == 2020 {
                    return Some((*a, *b, *c, a * b * c));
                }
            }
        }
    }
    None
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("err reading the file");
    let v: Vec<i64> = contents.lines().map(|l| l.parse().unwrap()).collect();

    match mult_of_2020sum_2(&v) {
        Some((n1, n2, p)) => println!("{} * {} = {}", n1, n2, p),
        None => println!("not found!"),
    }

    match mult_of_2020sum_3(&v) {
        Some((n1, n2, n3, p)) => println!("{} * {} * {} = {}", n1, n2, n3, p),
        None => println!("not found!"),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample_2() {
        let v: Vec<i64> = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(mult_of_2020sum_2(&v), Some((1721, 299, 514579)));
    }

    #[test]
    fn test_sample_3() {
        let v: Vec<i64> = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(mult_of_2020sum_3(&v), Some((979, 366, 675, 241861950)));
    }
}