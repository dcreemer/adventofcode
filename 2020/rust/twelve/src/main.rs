// solution to
// https://adventofcode.com/2020/day/12

use std::fs;

#[derive(Debug, PartialEq)]
enum Action {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

impl Action {
    fn parse(s: &str) -> Action {
        let v = s[1..].parse::<i32>().unwrap();
        match s.chars().next().unwrap() {
            'N' => Action::North(v),
            'S' => Action::South(v),
            'E' => Action::East(v),
            'W' => Action::West(v),
            'L' => Action::Left(v),
            'R' => Action::Right(v),
            'F' => Action::Forward(v),
            _ => panic!("unknown char"),
        }
    }
}

#[derive(Debug)]
struct Ship {
    x: i32,
    y: i32,
    dir: i32,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            x: 0,
            y: 0,
            dir: 90,
        }
    }

    fn apply_one(&mut self, a: &Action) {
        match a {
            Action::North(v) => self.y -= v,
            Action::South(v) => self.y += v,
            Action::East(v) => self.x += v,
            Action::West(v) => self.x -= v,
            Action::Left(v) => self.dir = self.dir - v + 360,
            Action::Right(v) => self.dir += v,
            Action::Forward(v) => match self.dir {
                0 => self.y -= v,
                90 => self.x += v,
                180 => self.y += v,
                270 => self.x -= v,
                _ => panic!("unknownd dir!"),
            },
        }
        self.dir %= 360;
    }
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }
    // rotate self about the center d degrees clockwise
    fn rotate(&mut self, d: i32) {
        let x = self.x;
        match d {
            0 => {}
            90 => {
                self.x = -self.y;
                self.y = x;
            }
            180 => {
                self.x = -self.x;
                self.y = -self.y;
            }
            270 => {
                self.x = self.y;
                self.y = -x;
            }
            _ => panic!("invalid rotation!"),
        }
    }

    fn forward(&mut self, target: &Position) {
        self.x += target.x;
        self.y += target.y;
    }

    fn apply(&mut self, a: &Action) {
        match a {
            Action::North(v) => self.y -= v,
            Action::South(v) => self.y += v,
            Action::East(v) => self.x += v,
            Action::West(v) => self.x -= v,
            Action::Left(v) => self.rotate(360 - *v),
            Action::Right(v) => self.rotate(*v),
            _ => panic!("invalid action"),
        }
    }
}

fn parse(data: &str) -> Vec<Action> {
    data.split('\n').map(|d| Action::parse(d)).collect()
}

fn part_1(data: &str) -> i32 {
    let mut s = Ship::new();
    parse(data).iter().for_each(|a| s.apply_one(a));
    s.x.abs() + s.y.abs()
}

fn part_2(data: &str) -> i32 {
    let mut ship = Position::new(0, 0);
    let mut waypoint = Position::new(10, -1);
    parse(data).iter().for_each(|a| {
        if let Action::Forward(n) = a {
            (0..*n).for_each(|_| ship.forward(&waypoint));
        } else {
            waypoint.apply(a);
        }
    });
    ship.x.abs() + ship.y.abs()
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("err reading the file");
    println!("part 1 = {:#?}", part_1(&contents));
    println!("part 2 = {:#?}", part_2(&contents));
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = "F10
N3
F7
R90
F11";

    #[test]
    fn test_part1() {
        let actions = parse(DATA);
        assert_eq!(actions[0], Action::Forward(10));
        assert_eq!(part_1(DATA), 25);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(DATA), 286);
    }
}
