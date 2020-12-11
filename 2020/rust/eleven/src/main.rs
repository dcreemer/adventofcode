// solution to
// https://adventofcode.com/2020/day/11

use std::cmp;
use std::fmt;
use std::fmt::Write;
use std::fs;

/// A grid Square is either empty or has an empty or occupied seat
#[derive(Clone, Copy, Debug, PartialEq)]
enum Square {
    Floor,
    Empty,
    Occupied,
}

// A grid is a 2d map, where x,y (0,0) is the top left
#[derive(Clone, Debug)]
struct Grid {
    width: usize,
    height: usize,
    data: Vec<Square>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Grid {
        let data = vec![Square::Empty; width * height];
        Grid {
            width,
            height,
            data,
        }
    }

    fn parse(raw: &str) -> Grid {
        let height = raw.lines().count();
        let width = raw.lines().next().unwrap().len();
        let mut g = Grid::new(width, height);
        for (y, line) in raw.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '.' => g.set_xy(x, y, Square::Floor),
                    'L' => g.set_xy(x, y, Square::Empty),
                    '#' => g.set_xy(x, y, Square::Occupied),
                    _ => panic!("bad input"),
                }
            }
        }
        g
    }

    fn neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        let xmin = if x == 0 { 0 } else { x - 1 };
        let ymin = if y == 0 { 0 } else { y - 1 };
        for i in xmin..=cmp::min(x + 1, self.width - 1) {
            for j in ymin..=cmp::min(y + 1, self.height - 1) {
                if i != x || j != y {
                    result.push((i, j));
                }
            }
        }
        result
    }

    fn occupied_neighbors(&self, x: usize, y: usize) -> usize {
        self.neighbors(x, y)
            .iter()
            .filter(|(i, j)| self.get_xy(*i, *j) == Square::Occupied)
            .count()
    }

    // this is horrible:
    fn visible_neighbors(&self, x: usize, y: usize) -> usize {
        let mut r = 0;
        // up:
        let mut j = y as i32 - 1;
        while j >= 0 {
            match self.get_xy(x, j as usize) {
                Square::Floor => j -= 1,
                Square::Empty => break,
                Square::Occupied => {
                    r += 1;
                    break;
                }
            }
        }
        // down:
        let mut j = y + 1;
        while j < self.height {
            match self.get_xy(x, j) {
                Square::Floor => j += 1,
                Square::Empty => break,
                Square::Occupied => {
                    r += 1;
                    break;
                }
            }
        }
        // left:
        let mut i = x as i32 - 1;
        while i >= 0 {
            match self.get_xy(i as usize, y) {
                Square::Floor => i -= 1,
                Square::Empty => break,
                Square::Occupied => {
                    r += 1;
                    break;
                }
            }
        }
        // right:
        let mut i = x + 1;
        while i < self.width {
            match self.get_xy(i, y) {
                Square::Floor => i += 1,
                Square::Empty => break,
                Square::Occupied => {
                    r += 1;
                    break;
                }
            }
        }
        // up left:
        let mut i = x as i32 - 1;
        let mut j = y as i32 - 1;
        while i >= 0 && j >= 0 {
            match self.get_xy(i as usize, j as usize) {
                Square::Floor => {
                    i -= 1;
                    j -= 1;
                }
                Square::Empty => break,
                Square::Occupied => {
                    r += 1;
                    break;
                }
            }
        }
        // up right:
        let mut i = x + 1;
        let mut j = y as i32 - 1;
        while i <self.width && j >= 0 {
            match self.get_xy(i, j as usize) {
                Square::Floor => {
                    i += 1;
                    j -= 1;
                }
                Square::Empty => break,
                Square::Occupied => {
                    r += 1;
                    break;
                }
            }
        }
        // down left:
        let mut i = x as i32 - 1;
        let mut j = y + 1;
        while i >= 0 && j < self.height {
            match self.get_xy(i as usize, j) {
                Square::Floor => {
                    i -= 1;
                    j += 1;
                }
                Square::Empty => break,
                Square::Occupied => {
                    r += 1;
                    break;
                }
            }
        }
        // down right:
        let mut i = x + 1;
        let mut j = y + 1;
        while i < self.width && j < self.height {
            match self.get_xy(i, j) {
                Square::Floor => {
                    i += 1;
                    j += 1;
                }
                Square::Empty => break,
                Square::Occupied => {
                    r += 1;
                    break;
                }
            }
        }
        r
    }

    // I don't like passing 'o' to determine visible vs. occupied neighbors.
    // change to pass a lambda
    fn transform(&self, threshold: usize, o: bool) -> Grid {
        let mut r = Grid::new(self.width, self.height);
        for x in 0..self.width {
            for y in 0..self.height {
                let state = self.get_xy(x, y);
                let n = if o {
                    self.occupied_neighbors(x, y)
                } else {
                    self.visible_neighbors(x, y)
                };
                let newstate = match state {
                    Square::Floor => Square::Floor,
                    Square::Empty => {
                        if n == 0 {
                            Square::Occupied
                        } else {
                            Square::Empty
                        }
                    }
                    Square::Occupied => {
                        if n >= threshold {
                            Square::Empty
                        } else {
                            Square::Occupied
                        }
                    }
                };
                r.set_xy(x, y, newstate);
            }
        }
        r
    }

    fn occupied(&self) -> usize {
        let mut r = 0;
        for x in 0..self.width {
            for y in 0..self.height {
                if self.get_xy(x, y) == Square::Occupied {
                    r += 1;
                }
            }
        }
        r
    }

    fn xy(&self, x: usize, y: usize) -> usize {
        y * self.width + x % self.width
    }
    fn get_xy(&self, x: usize, y: usize) -> Square {
        self.data[self.xy(x, y)]
    }

    fn set_xy(&mut self, x: usize, y: usize, v: Square) {
        let idx = self.xy(x, y);
        self.data[idx] = v;
    }
}

impl PartialEq for Grid {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = String::with_capacity(self.height * (self.width + 1));
        for y in 0..self.height {
            for x in 0..self.width {
                match self.get_xy(x, y) {
                    Square::Floor => write!(&mut res, ".").unwrap(),
                    Square::Empty => write!(&mut res, "L").unwrap(),
                    Square::Occupied => write!(&mut res, "#").unwrap(),
                }
            }
            writeln!(&mut res).unwrap();
        }
        write!(f, "{}", res)
    }
}

fn part1(g: &Grid) -> usize {
    let mut g = g.clone();
    loop {
        let xg = g.transform(4, true);
        if g == xg {
            return g.occupied();
        }
        g = xg;
    }
}

fn part2(g: &Grid) -> usize {
    let mut g = g.clone();
    loop {
        let xg = g.transform(5, false);
        if g == xg {
            return g.occupied();
        }
        g = xg;
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("err reading the file");
    let g = Grid::parse(&contents);
    println!("{}", g);
    println!("{}", part1(&g));
    println!("{}", part2(&g));
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn test_part1() {
        let g = Grid::parse(DATA);
        assert_eq!(part1(&g), 37);
    }

    #[test]
    fn test_part2() {
        let g = Grid::parse(DATA);
        assert_eq!(part2(&g), 26);
    }
}
