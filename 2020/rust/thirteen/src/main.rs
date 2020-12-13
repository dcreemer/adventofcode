// solution to
// https://adventofcode.com/2020/day/13

use std::fs;

fn parse_1(data: &str) -> (i64, Vec<i64>) {
    let l: Vec<&str> = data.lines().collect();
    let ts = l[0].parse::<i64>().unwrap();
    let tms: Vec<i64> = l[1]
        .split(',')
        .filter(|&e| e != "x")
        .map(|e| e.parse::<i64>().unwrap())
        .collect();
    (ts, tms)
}

fn part_1(data: &str) -> i64 {
    let (tm, tms) = parse_1(data);
    eprintln!("{}\n{:?}", tm, tms);
    let mut t = tm;
    loop {
        for b in tms.iter() {
            if t % b == 0 {
                return (t - tm) * *b;
            }
        }
        t += 1;
    }
}

/// chinese_remainder impl from rosetta code.
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();
    let mut sum = 0;
    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }
    Some(sum % prod)
}

fn parse_2(data: &str) -> Vec<i64> {
    data.lines()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|e| {
            if e == "x" {
                1
            } else {
                e.parse::<i64>().unwrap()
            }
        })
        .collect()
}

fn part_2(data: &str) -> i64 {
    // find the "chinese_remainder" of the bus ids, where the first one has a
    // residue (mod) of zero, as does any bux with a id of "x" (bus ID 1).
    // Subsequent busses have a remainder of the ID - their array index.
    let modulii = parse_2(data);
    let residues: Vec<i64> = modulii
        .iter()
        .enumerate()
        .map(|(i, v)| {
            if i == 0 || *v == 1 {
                0_i64
            } else {
                *v - (i as i64)
            }
        })
        .collect();
    if let Some(v) = chinese_remainder(&residues, &modulii) {
        return v;
    }
    panic!("no solution!");
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("err reading the file");
    println!("part 1 = {:#?}", part_1(&contents));
    println!("part 2 = {:#?}", part_2(&contents));
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = "939
7,13,x,x,59,x,31,19";

    #[test]
    fn test_part1() {
        let (tm, tms) = parse_1(DATA);
        assert_eq!(tm, 939);
        assert_eq!(tms.len(), 5);
        assert_eq!(tms[0], 7);
        assert_eq!(tms[2], 59);
        assert_eq!(part_1(DATA), 295);
    }

    #[test]
    fn test_part2() {
        let tms = parse_2(DATA);
        assert_eq!(tms.len(), 8);
        assert_eq!(tms[0], 7);
        assert_eq!(tms[2], 1);
        assert_eq!(part_2(DATA), 1068781);
        assert_eq!(part_2("0\n17,16"), 255);
        assert_eq!(part_2("0\n17,x,13,19"), 3417);
        assert_eq!(part_2("0\n67,7,59,61"), 754018);
        assert_eq!(part_2("0\n67,x,7,59,61"), 779210);
        assert_eq!(part_2("0\n67,7,x,59,61"), 1261476);
        assert_eq!(part_2("0\n1789,37,47,1889"), 1_202_161_486);
    }
}
