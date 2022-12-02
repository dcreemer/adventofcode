// solution to
// https://adventofcode.com/2022/day/2

use std::collections::HashMap;
use std::fs;

#[derive(Clone, Copy, Debug)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Lose,
    Draw,
    Win,
}

type Game = Vec<(Choice, Choice)>;

fn score_round(other: &Choice, me: &Choice) -> i64 {
    match (me, other) {
        (Choice::Rock, Choice::Rock) => 1 + 3,
        (Choice::Rock, Choice::Paper) => 1 + 0,
        (Choice::Rock, Choice::Scissors) => 1 + 6,
        (Choice::Paper, Choice::Rock) => 2 + 6,
        (Choice::Paper, Choice::Paper) => 2 + 3,
        (Choice::Paper, Choice::Scissors) => 2 + 0,
        (Choice::Scissors, Choice::Rock) => 3 + 0,
        (Choice::Scissors, Choice::Paper) => 3 + 6,
        (Choice::Scissors, Choice::Scissors) => 3 + 3,
    }
}

fn choose_mine(other: &Choice, outcome: &Outcome) -> Choice {
    match (other, outcome) {
        (Choice::Rock, Outcome::Lose) => Choice::Scissors,
        (Choice::Rock, Outcome::Draw) => Choice::Rock,
        (Choice::Rock, Outcome::Win) => Choice::Paper,
        (Choice::Paper, Outcome::Lose) => Choice::Rock,
        (Choice::Paper, Outcome::Draw) => Choice::Paper,
        (Choice::Paper, Outcome::Win) => Choice::Scissors,
        (Choice::Scissors, Outcome::Lose) => Choice::Paper,
        (Choice::Scissors, Outcome::Draw) => Choice::Scissors,
        (Choice::Scissors, Outcome::Win) => Choice::Rock,
    }
}

fn score_game(game: &Game) -> i64 {
    game.iter().map(|(m, o)| score_round(m, o)).sum()
}

fn parse_1(data: &str) -> Game {
    let lookup = HashMap::from([
        ('A', Choice::Rock),
        ('B', Choice::Paper),
        ('C', Choice::Scissors),
        ('X', Choice::Rock),
        ('Y', Choice::Paper),
        ('Z', Choice::Scissors),
    ]);

    data.lines()
        .map(|line| {
            let o = line.chars().nth(0).unwrap();
            let m = line.chars().nth(2).unwrap();
            (lookup[&o], lookup[&m])
        })
        .collect()
}

fn parse_2(data: &str) -> Game {
    let choice = HashMap::from([
        ('A', Choice::Rock),
        ('B', Choice::Paper),
        ('C', Choice::Scissors),
    ]);
    let outcome = HashMap::from([
        ('X', Outcome::Lose),
        ('Y', Outcome::Draw),
        ('Z', Outcome::Win),
    ]);

    data.lines()
        .map(|line| {
            let o = line.chars().nth(0).unwrap();
            let other = choice[&o];
            let oc = line.chars().nth(2).unwrap();
            let me = choose_mine(&other, &outcome[&oc]);
            (other, me)
        })
        .collect()
}

fn main() {
    let data = fs::read_to_string("input.txt").expect("err reading the file");
    let game1 = parse_1(&data);
    println!("part 1 = {}", score_game(&game1));
    let game2 = parse_2(&data);
    println!("part 2 = {}", score_game(&game2));
}

#[cfg(test)]
mod test {
    use crate::*;

    static DATA: &'static str = include_str!("test_data.txt");

    #[test]
    fn test_score_round() {
        assert_eq!(score_round(&Choice::Rock, &Choice::Scissors), 3);
        assert_eq!(score_round(&Choice::Scissors, &Choice::Scissors), 6);
        assert_eq!(score_round(&Choice::Paper, &Choice::Scissors), 9);
    }

    #[test]
    fn test_score_game() {
        let g = vec![
            (Choice::Rock, Choice::Scissors),
            (Choice::Scissors, Choice::Scissors),
            (Choice::Paper, Choice::Scissors),
        ];
        assert_eq!(score_game(&g), 18);
    }

    #[test]
    fn test_sample() {
        let game_1 = parse_1(DATA);
        assert_eq!(score_game(&game_1), 15);
        let game_2 = parse_2(DATA);
        assert_eq!(score_game(&game_2), 12);
    }
}
