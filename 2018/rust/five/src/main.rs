use std::fs;

// remove any adjacent pairs (e.g. 'a' 'A') in the input
fn react_once(s: &str) -> String {
    let mut result = String::new();
    // tack on a space to test the last two letters
    let s = format!("{} ", s);
    let mut iter = s.chars().peekable();
    while let Some(c) = iter.next() {
        if let Some(&n) = iter.peek() {
            if c != n && (c.to_ascii_uppercase() == n.to_ascii_uppercase()) {
                iter.next();
                continue;
            } else {
                result.push(c);
            }
        }
    }
    // if the final result ends in a space, remove it
    if result.len() > 0 {
        if let Some(' ') = result.chars().nth(result.len() - 1) {
            result.pop();
        }
    }
    result
}

// remove any adjacent pair (e.g. 'a' 'A') from the input until all pairs are
// gone.
fn react(s: &str) -> String {
    let mut a = s.to_owned();
    loop {
        let b = react_once(&a);
        if a == b {
            // input matches the output - nothing more to react
            return a;
        }
        a = b;
    }
}

static UNITS: [(char, char); 26] = [
    ('a', 'A'),
    ('b', 'B'),
    ('c', 'C'),
    ('d', 'D'),
    ('e', 'E'),
    ('f', 'F'),
    ('g', 'G'),
    ('h', 'H'),
    ('i', 'I'),
    ('j', 'J'),
    ('k', 'K'),
    ('l', 'L'),
    ('m', 'M'),
    ('n', 'N'),
    ('o', 'O'),
    ('p', 'P'),
    ('q', 'Q'),
    ('r', 'R'),
    ('s', 'S'),
    ('t', 'T'),
    ('u', 'U'),
    ('v', 'V'),
    ('w', 'W'),
    ('x', 'X'),
    ('y', 'Y'),
    ('z', 'Z'),
];

// find the shortest reaction resulting from removing any single pair (e.g. 'a'
// 'A') from the input sequence.
fn shortest(s: &str) -> String {
    let mut shortest = s.to_owned();
    for (lc, uc) in UNITS.iter() {
        let cand: String = react(&s.chars().filter(|c| c != lc && c != uc).collect::<String>());
        if cand.len() < shortest.len() {
            shortest = cand;
        }
    }
    shortest
}

fn main() {
    println!("AoC 2018 Five");
    let content = fs::read_to_string("input.txt")
        .expect("err reading the file")
        .trim()
        .to_owned();
    println!("{:#?}", react(&content).len());
    println!("{:#?}", shortest(&content).len());
}

mod test {

    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_react() {
        assert_eq!(react("aA"), "");
        assert_eq!(react("aAbB"), "");
        assert_eq!(react("abBA"), "");
        assert_eq!(react("abAB"), "abAB");
        assert_eq!(react("aabAAB"), "aabAAB");
        assert_eq!(react("dabAcCaCBAcCcaDA"), "dabCBAcaDA");
    }

    #[test]
    fn test_shortest() {
        assert_eq!(shortest("dabAcCaCBAcCcaDA"), "daDA");
    }
}
