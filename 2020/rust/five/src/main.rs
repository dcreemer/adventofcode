// solution to
// https://adventofcode.com/2020/day/5
use std::fs;

fn str_to_binary(data: &str) -> i32 {
    let bstr: String = data
        .chars()
        .map(|c| match c {
            'F' => '0',
            'L' => '0',
            'B' => '1',
            'R' => '1',
            _ => '0',
        })
        .collect();
    i32::from_str_radix(&bstr, 2).unwrap()
}

fn main() {
    let mut entries: Vec<i32> = fs::read_to_string("input.txt")
        .expect("err reading the file")
        .lines()
        .map(|l| str_to_binary(&l[0..7]) * 8 + str_to_binary(&l[7..10]))
        .collect();

    // find max:
    println!("part 1: {}", entries.iter().max().unwrap());

    // sort and find hole:
    entries.sort_unstable();
    println!(
        "part 2: {:?}",
        entries
            .iter()
            .zip(entries.iter().skip(1))
            .find(|(&l, &h)| (h - l) == 2)
            .unwrap()
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(str_to_binary("BFFFBBF"), 70)
    }
}
