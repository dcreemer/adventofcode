// solution to
// https://adventofcode.com/2020/day/1

use std::fmt;
use std::fmt::Write;
use std::fs;

/// A grid Square is either empty or has a tree
#[derive(Clone, Copy, Debug, PartialEq)]
enum Square {
    Empty,
    Tree,
}

// A grid is a 2d map, where x,y (0,0) is the top left, and the grid
// repeats infinitely horizontally (i.e. the grid is a cylinder)
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
                if c == '#' {
                    g.set_xy(x, y, Square::Tree);
                }
            }
        }
        g
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

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = String::with_capacity(self.height * (self.width + 1));
        for y in 0..self.height {
            for x in 0..self.width {
                match self.get_xy(x, y) {
                    Square::Empty => write!(&mut res, ".").unwrap(),
                    Square::Tree => write!(&mut res, "#").unwrap(),
                }
            }
            writeln!(&mut res).unwrap();
        }
        write!(f, "{}", res)
    }
}

// "Walk" the grid from the top-left, moving down and to the right the
// given amounts. Count the number of Square::Tree's encountered, until
// we reach the bottom
fn walk(g: &Grid, right: usize, down: usize) -> i32 {
    let mut trees = 0;
    let mut x = 0;
    let mut y = 0;
    while y < g.height {
        if g.get_xy(x, y) == Square::Tree {
            trees += 1;
        }
        y += down;
        x += right;
    }
    trees
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("err reading the file");
    let g = Grid::parse(&contents);
    println!("part  1: {}", walk(&g, 3, 1));
    let walks = vec![walk(&g, 1, 1), walk(&g, 3, 1), walk(&g, 5, 1), walk(&g, 7, 1), walk(&g, 1, 2)];
    println!("part  2: {:?}", walks);
    println!("product: {}", walks.iter().fold(1, |acc:i64, x| acc * *x as i64));
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn test_grid() {
        let m = Grid::parse(DATA);
        assert_eq!(m.width, 11);
        assert_eq!(m.height, 11);
        assert_eq!(m.get_xy(0, 0), Square::Empty);
        assert_eq!(m.get_xy(3, 0), Square::Tree);
        assert_eq!(m.get_xy(4, 0), Square::Empty);
        assert_eq!(m.get_xy(3+11, 0), Square::Tree);
        assert_eq!(m.get_xy(3+22, 0), Square::Tree);
        assert_eq!(m.get_xy(4+11, 0), Square::Empty);
        assert_eq!(m.get_xy(4+22, 0), Square::Empty);
        assert_eq!(m.get_xy(0, 1), Square::Tree);
    }

    #[test]
    fn test_walk_1() {
        let g = Grid::parse(DATA);
        assert_eq!(walk(&g, 3, 1), 7);
    }

    #[test]
    fn test_walk_2() {
        let g = Grid::parse(DATA);
        assert_eq!(walk(&g, 1, 1), 2);
        assert_eq!(walk(&g, 3, 1), 7);
        assert_eq!(walk(&g, 5, 1), 3);
        assert_eq!(walk(&g, 7, 1), 4);
        assert_eq!(walk(&g, 1, 2), 2);
    }
}
