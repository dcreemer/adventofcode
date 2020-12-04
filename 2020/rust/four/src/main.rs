// solution to
// https://adventofcode.com/2020/day/4

use std::collections::HashMap;
use std::fs;
use lazy_static::lazy_static;
use regex::Regex;

/// model passport Record as a HashMap
#[derive(PartialEq, Debug)]
struct Record(HashMap<String, String>);

impl Record {
    /// parse "k1:v1 k2:v2 ..."
    fn parse(data: &str) -> Record {
        let m: HashMap<String, String> = data
            .split_whitespace()
            .map(|e| {
                let p: Vec<&str> = e.split(':').collect();
                (p[0].to_string(), p[1].to_string())
            })
            .collect();
        Record(m)
    }

    fn keys_valid(&self) -> bool {
        ["ecl", "pid", "eyr", "hcl", "byr", "iyr", "hgt"]
            .iter()
            .map(|s| s.to_string())
            .all(|k| self.0.contains_key(&k))
    }

    fn values_valid(&self) -> bool {
        self.keys_valid()
            && year_valid(self.0.get("byr").unwrap(), 1920, 2002)
            && year_valid(self.0.get("iyr").unwrap(), 2010, 2020)
            && year_valid(self.0.get("eyr").unwrap(), 2020, 2030)
            && hgt_valid(self.0.get("hgt").unwrap())
            && hcl_valid(self.0.get("hcl").unwrap())
            && ecl_valid(self.0.get("ecl").unwrap())
            && pid_valid(self.0.get("pid").unwrap())
    }
}

fn year_valid(data: &str, min: i32, max: i32) -> bool {
    if let Ok(year) = data.parse::<i32>() {
        return year >= min && year <= max;
    }
    false
}

fn hgt_valid(data: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d{2,3})(in|cm)$").unwrap();
    }
    if let Some(cap) = RE.captures(data) {
        let h = cap.get(1).unwrap().as_str().parse::<i32>();
        let t = cap.get(2).unwrap().as_str();
        return match (h, t) {
            (Ok(h), "cm") => h >= 150 && h <= 193,
            (Ok(h), "in") => h >= 59 && h <= 76,
            _ => false
        }
    }
    false
}

fn hcl_valid(data: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    }
    RE.is_match(data)
}

fn ecl_valid(data: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
    }
    RE.is_match(data)
}

fn pid_valid(data: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\d{9}$").unwrap();
    }
    RE.is_match(data)
}

fn parse(data: &str) -> Vec<Record> {
    data.split("\n\n").map(|s| Record::parse(s)).collect()
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("err reading the file");
    let v = parse(&contents);
    println!(
        "part 1:  keys valid = {}",
        v.iter().filter(|r| r.keys_valid()).count()
    );
    println!(
        "part 2: value valid = {}",
        v.iter().filter(|r| r.values_valid()).count()
    );
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    #[test]
    fn test_parse() {
        let records = parse(DATA);
        assert!(records[0].0.get("ecl") == Some(&"gry".to_string()));
        assert!(records[1].0.get("hcl") == Some(&"#cfa07d".to_string()));
    }

    #[test]
    fn test_valid_keys() {
        let records = parse(DATA);
        assert!(records[0].keys_valid());
        assert!(!records[1].keys_valid());
        assert!(records[2].keys_valid());
        assert!(!records[3].keys_valid());
    }
}
