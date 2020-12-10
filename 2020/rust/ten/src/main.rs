// solution to
// https://adventofcode.com/2020/day/10

use std::fs;

fn parse(data: &str) -> Vec<i64> {
    data.lines().map(|l| l.parse::<i64>().unwrap()).collect()
}

fn part1(input: &Vec<i64>) -> i64 {
    let mut data = input.clone();
    data.sort_unstable();
    data.push(data[data.len() - 1] + 3);
    let (mut s1, mut s3) = (0, 0);
    let _ = data.iter().fold(0, |prev, i| {
        match i - prev {
            1 => s1 += 1,
            3 => s3 += 1,
            _ => {
                eprintln!("err: {} {}", prev, i)
            }
        }
        *i
    });
    s1 * s3
}

fn part2(input: &Vec<i64>) -> i64 {
    let mut data = input.clone();
    data.sort_unstable();
    data.insert(0, 0);
    data.push(data[data.len() - 1] + 3);
    let mut idx = 0;
    let mut result = 1;
    eprintln!("v={:?}", data);
    // find runs of 3, 4, or 5 numbers in a row that do not skip more than 3 ahead
    while idx < data.len() {
        let mut t = data[idx] + 3;
        let mut run_size = 1;
        while idx < data.len() - 1 && data[idx + 1] < t {
            run_size += 1;
            idx += 1;
            t = data[idx] + 3;
        }
        idx += 1;
        // run_size of 3 has 2 combinations, 4 has 4 combinations, and 5 has 7 combinations
        match run_size {
            1 => {},
            2 => {},
            3 => {
                result *= 2;
            }
            4 => {
                result *= 4;
            }
            5 => {
                result *= 7;
            }
            _ => {
                panic!("bad assumption > 4, <2");
            }
        }
    }
    result
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("err reading the file");
    let v = parse(&contents);
    println!("part 1 = {:?}", part1(&v));
    println!("part 2 = {:?}", part2(&v));
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = "16
10
15
5
1
11
7
19
6
12
4";

    const DATA2: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    fn test_part1() {
        let v = parse(DATA);
        assert_eq!(part1(&v), 35);
        let v = parse(DATA2);
        assert_eq!(part1(&v), 220);
    }

    #[test]
    fn test_part2() {
        let v = parse(DATA);
        assert_eq!(part2(&v), 8);
        let v = parse(DATA2);
        assert_eq!(part2(&v), 19208);
    }
}
