use regex::Regex;
use std::fs;

#[macro_use]
extern crate lazy_static;

type Matrix = Box<[[u8; 1000]]>;

#[derive(Debug, PartialEq)]
struct Claim {
    n: u16,
    x: u16,
    y: u16,
    w: u16,
    h: u16,
}

impl Claim {
    // parse a claim - note assume it will not fail
    fn from_str(s: &str) -> Self {
        lazy_static! {
            static ref re: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
        }
        let caps = re.captures(s).unwrap();
        Claim {
            n: caps[1].parse::<u16>().unwrap(),
            x: caps[2].parse::<u16>().unwrap(),
            y: caps[3].parse::<u16>().unwrap(),
            w: caps[4].parse::<u16>().unwrap(),
            h: caps[5].parse::<u16>().unwrap(),
        }
    }

    fn apply_to(&self, m: &mut Matrix) {
        for row in (self.y)..(self.y + self.h) {
            for col in self.x..(self.x + self.w) {
                m[row as usize][col as usize] += 1;
            }
        }
    }

    fn is_solid(&self, m: &Matrix) -> bool {
        for row in (self.y)..(self.y + self.h) {
            for col in self.x..(self.x + self.w) {
                if m[row as usize][col as usize] > 1 {
                    return false;
                }
            }
        }
        true
    }
}

// find count of cells with more than one claim:
fn part1(m: &Matrix) -> i32 {
    let mut s = 0;
    for row in 0..1000 {
        for col in 0..1000 {
            if m[row as usize][col as usize] > 1 {
                s += 1
            }
        }
    }
    s
}

// find the first claim that is solid, or None
fn part2(claims: &Vec<Claim>, m: &Matrix) -> Option<u16> {
    for c in claims.iter() {
        if c.is_solid(&m) {
            return Some(c.n);
        }
    }
    None
}

fn main() {
    println!("AoC 2018 Three");

    // create the matrix, and apply all claims to it
    let mut matrix: Matrix = Box::new([[0u8; 1000]; 1000]);
    let claims: Vec<Claim> = fs::read_to_string("input.txt")
        .expect("err reading the file")
        .lines()
        .map(|l| Claim::from_str(l))
        .collect();
    claims.iter().for_each(|c| c.apply_to(&mut matrix));

    println!("part1 = {}", part1(&matrix));
    println!("part2 = {}", part2(&claims, &matrix).unwrap());
}

mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_parse_line() {
        let text = "#1 @ 249,597: 20x15";
        assert_eq!(
            Claim::from_str(text),
            Claim {
                n: 1,
                x: 249,
                y: 597,
                w: 20,
                h: 15,
            }
        );
    }
}
