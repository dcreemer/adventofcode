// solution to
// https://adventofcode.com/2020/day/2

use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

#[derive(Debug, PartialEq)]
struct Entry {
    a: usize,
    b: usize,
    ch: char,
    pass: String,
}

impl Entry {
    fn parse(input: &str) -> Option<Entry> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(?P<a>\d+)-(?P<b>\d+) (?P<ch>[[:alpha:]]): (?P<pass>[[:alpha:]]+)$")
                    .unwrap();
        }
        if let Some(cap) = RE.captures(input) {
            // since the RE parses, all of the captures will meet the spec, and
            // so the "unwrap()" calls are safe.
            return Some(Entry {
                a: cap.name("a").unwrap().as_str().parse().unwrap(),
                b: cap.name("b").unwrap().as_str().parse().unwrap(),
                ch: cap.name("ch").unwrap().as_str().chars().next().unwrap(),
                pass: cap.name("pass").unwrap().as_str().to_owned(),
            });
        }
        None
    }

    fn validate_one(&self) -> bool {
        let ch_count = self.pass.chars().filter(|c| *c == self.ch).count();
        self.a <= ch_count && self.b >= ch_count
    }

    fn validate_two(&self) -> bool {
        if self.b <= self.pass.len() {
            let a = self.pass.chars().nth(self.a - 1).unwrap();
            let b = self.pass.chars().nth(self.b - 1).unwrap();
            return (a == self.ch || b == self.ch) && !(a == self.ch && b == self.ch);
        }
        false
    }
}

fn main() {
    // will panic on parse errors:
    let entries: Vec<Entry> = fs::read_to_string("input.txt")
        .expect("err reading the file")
        .lines()
        .map(|l| Entry::parse(l).unwrap())
        .collect();

    println!(
        "part 1, 2 valid count: {}, {}",
        entries.iter().filter(|e| e.validate_one()).count(),
        entries.iter().filter(|e| e.validate_two()).count()
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_line() {
        let e = Entry::parse("1-2 o: foo");
        assert_eq!(
            e,
            Some(Entry {
                a: 1,
                b: 2,
                ch: 'o',
                pass: "foo".to_string()
            })
        );
        // these should not parse
        let e = Entry::parse("foo");
        assert_eq!(e, None);
        let e = Entry::parse("1-2: foo");
        assert_eq!(e, None);
        let e = Entry::parse("1-2 foo");
        assert_eq!(e, None);
    }

    #[test]
    fn test_validate_one() {
        let e = Entry::parse("1-3 a: abcde").unwrap();
        assert!(e.validate_one());
        let e = Entry::parse("1-3 b: cdefg").unwrap();
        assert!(!e.validate_one());
        let e = Entry::parse("2-9 c: ccccccccc").unwrap();
        assert!(e.validate_one());
    }

    #[test]
    fn test_validate_two() {
        let e = Entry::parse("1-3 a: abcde").unwrap();
        assert!(e.validate_two());
        let e = Entry::parse("1-3 b: cdefg").unwrap();
        assert!(!e.validate_two());
        let e = Entry::parse("2-9 c: ccccccccc").unwrap();
        assert!(!e.validate_two());
    }
}
