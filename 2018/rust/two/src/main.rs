use std::collections::HashMap;
use std::fs;

// given a work (as a string slice), return a map of each unique
// character in the string, along wiht a frequencies count.
fn letter_frequencies(word: &str) -> HashMap<char, u32> {
    let mut letters = HashMap::new();
    for c in word.chars() {
        let counter = letters.entry(c).or_insert(0);
        *counter += 1;
    }
    letters
}

// the hash is the count of strings with 2-frequencies times the count
// of strings with 3-frequencies
fn compute_hash(fms: &Vec<HashMap<char, u32>>) -> u32 {
    let (twos, threes) = fms.iter().fold((0, 0), |acc, fm| {
        (
            acc.0 + if fm.values().any(|&x| x == 2) { 1 } else { 0 },
            acc.1 + if fm.values().any(|&x| x == 3) { 1 } else { 0 },
        )
    });
    twos * threes
}

// given two strings, if the strings are off by *exactly* one letter,
// then return that string; If not, return None
fn off_by_one(a: &str, b: &str) -> Option<String> {
    let mut s = String::new();
    let mut one = false;
    for (ca, cb) in a.chars().zip(b.chars()) {
        if ca == cb {
            s.push(ca);
        } else if !one {
            one = true
        } else {
            return None;
        }
    }
    if one {
        Some(s)
    } else {
        None
    }
}

// given a string and a list of strings, return the first string that is
// "off_by_one" (as a new String with the off by one char missing), or None
fn find_off_by_one(s: &str, v: &[&str]) -> Option<String> {
    for c in v.iter() {
        if let Some(oos) = off_by_one(s, c) {
            return Some(oos);
        }
    }
    None
}

// examine each string in a list, looking for another string in the list
// that is off-by-one character. Return the off-by-one string, if found
fn find_first_off_by_one(v: &Vec<&str>) -> Option<String> {
    for (i, c) in v.iter().enumerate() {
        if let Some(oos) = find_off_by_one(c, &v[(i + 1)..]) {
            return Some(oos);
        }
    }
    None
}

fn main() {
    println!("AoC 2018 Two");
    let contents = fs::read_to_string("input.txt").expect("err reading the file");
    let lines: Vec<&str> = contents.lines().collect();
    let freqmaps: Vec<HashMap<char, u32>> = lines.iter().map(|s| letter_frequencies(s)).collect();
    println!("Hash {}", compute_hash(&freqmaps));
    if let Some(oos) = find_first_off_by_one(&lines) {
        println!("first off-by-one: {}", oos)
    } else {
        println!("no first off-by-one found!")
    }
}

mod test {
    use super::*;

    #[test]
    fn test_has_count() {
        let fm = letter_frequencies("hello");
        assert!(fm.values().any(|&x| x == 2));
    }

    #[test]
    fn test_hash() {
        let freqmaps: Vec<HashMap<char, u32>> = vec![
            "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
        ]
        .iter()
        .map(|s| letter_frequencies(s))
        .collect();
        assert_eq!(compute_hash(&freqmaps), 12);
    }

    #[test]
    fn test_off_by_one() {
        assert_eq!(off_by_one("abcde", "fghij"), None);
        assert_eq!(off_by_one("abcde", "abcde"), None);
        assert_eq!(off_by_one("fghij", "fguij"), Some("fgij".to_string()));
    }

    #[test]
    fn test_find_off_by_one() {
        let v = vec!["abcde", "fghij", "fguij"];
        assert_eq!(find_off_by_one("abcde", &v), None);
        assert_eq!(find_off_by_one("abcdf", &v), Some("abcd".to_string()));
    }

    #[test]
    fn test_find_first_off_by_one() {
        let v = vec!["abcde", "fghij", "fguij"];
        assert_eq!(find_first_off_by_one(&v), Some("fgij".to_string()));
    }
}
