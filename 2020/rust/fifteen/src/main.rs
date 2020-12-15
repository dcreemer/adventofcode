// solution to
// https://adventofcode.com/2020/day/15

use std::collections::HashMap;

fn solve(data: &str, upto: usize) -> i64 {
    let mut prevs: HashMap<i64, [i64;2]> = HashMap::new();
    let mut turn = 1;
    let seeds: Vec<i64> = data.split(',').map(|f| f.parse::<i64>().unwrap()).collect();
    let mut last = -1_i64;
    let mut fts = true;
    while turn <= upto {
        let next = if turn <= seeds.len() {
            seeds[turn-1]
        } else if fts {
            0
        } else {
            let turns = prevs.get(&last).unwrap();
            turns[1] - turns[0]
        };
        let turns = prevs.entry(next).or_insert([-1, -1]);
        fts = turns[1] == -1;
        turns[0] = turns[1];
        turns[1] = turn as i64;
        last = next;
        turn += 1;
    }
    last
}

fn main() {
    let contents = "1,20,11,6,12,0";
    println!("part 1 = {:#?}", solve(contents, 2020));
    println!("part 1 = {:#?}", solve(contents, 30000000));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve("0,3,6", 2020), 436);
        assert_eq!(solve("2,1,3", 2020), 10);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve("0,3,6", 30000000), 175594);
    }
}
