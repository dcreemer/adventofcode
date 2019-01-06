use std::collections::HashSet;
use std::fs;

fn freq_changes(changes: &Vec<i32>) -> i32 {
    changes.into_iter().sum()
}

fn first_seen(changes: &Vec<i32>) -> i32 {
    // like fold, but return the running sum
    let xs = changes.into_iter().cycle().scan(0, |sum, x| {
        *sum = *sum + x;
        Some(*sum)
    });
    let mut seen: HashSet<i32> = HashSet::new();
    seen.insert(0);
    for v in xs {
        if seen.contains(&v) {
            return v;
        }
        seen.insert(v);
    }
    0
}

fn main() {
    // read the input file into a Vec<i32>
    let contents = fs::read_to_string("input.txt").expect("err reading the file");
    let changes: Vec<i32> = contents.lines().map(|l| l.parse().unwrap()).collect();
    println!("freq_changes = {}", freq_changes(&changes));
    println!("first_seen = {}", first_seen(&changes));
}

#[cfg(test)]
mod testing {
    use crate::*;

    #[test]
    fn test_freq_changes() {
        assert_eq!(freq_changes(&vec![1, -2, 3, 1]), 3);
        assert_eq!(freq_changes(&vec![1, 1, 1]), 3);
        assert_eq!(freq_changes(&vec![-1, -2, -3]), -6);
    }

    #[test]
    fn test_first_seen() {
        assert_eq!(first_seen(&vec![1, -2, 3, 1]), 2);
        assert_eq!(first_seen(&vec![1, -1]), 0);
        assert_eq!(first_seen(&vec![3, 3, 4, -2, -4]), 10);
        assert_eq!(first_seen(&vec![-6, 3, 8, 5, -6]), 5);
        assert_eq!(first_seen(&vec![7, 7, -2, -7, -4]), 14);
    }
}
