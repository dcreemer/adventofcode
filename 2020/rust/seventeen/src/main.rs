// solution to
// https://adventofcode.com/2020/day/17

use std::cmp;
use std::fmt;
use std::fmt::Write;
use std::fs;

#[derive(Clone, Copy, Debug, PartialEq)]
enum State {
    Active,
    Inactive,
}

// A Space is a 3d map
#[derive(Clone, Debug)]
struct Space {
    dim_x: usize,
    dim_y: usize,
    dim_z: usize,
    data: Vec<State>,
}

impl Space {
    fn new(dim_x: usize, dim_y: usize, dim_z: usize) -> Space {
        let data = vec![State::Inactive; dim_x * dim_y * dim_z];
        Space {
            dim_x,
            dim_y,
            dim_z,
            data,
        }
    }

    fn parse(raw: &str) -> Space {
        let dim_x = raw.lines().count();
        let dim_y = raw.lines().next().unwrap().len();
        let mut s = Space::new(dim_x, dim_y, 1);
        for (y, line) in raw.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '.' => s.set_xyz(x, y, 0, State::Inactive),
                    '#' => s.set_xyz(x, y, 0, State::Active),
                    _ => panic!("bad input"),
                }
            }
        }
        s
    }

    fn active_count(&self) -> i64 {
        self.data.iter().filter(|&&s| s == State::Active).count() as i64
    }

    fn neighbors(&self, x: usize, y: usize, z: usize) -> Vec<(usize, usize, usize)> {
        let mut result = Vec::new();
        let xmin = if x == 0 { 0 } else { x - 1 };
        let ymin = if y == 0 { 0 } else { y - 1 };
        let zmin = if z == 0 { 0 } else { z - 1 };
        for i in xmin..=cmp::min(x + 1, self.dim_x - 1) {
            for j in ymin..=cmp::min(y + 1, self.dim_y - 1) {
                for k in zmin..=cmp::min(z + 1, self.dim_z - 1) {
                    if i != x || j != y || k != z {
                        result.push((i, j, k));
                    }
                }
            }
        }
        result
    }

    fn active_neighbors(&self, x: usize, y: usize, z: usize) -> usize {
        self.neighbors(x, y, z)
            .iter()
            .filter(|(i, j, k)| self.get_xyz(*i, *j, *k) == State::Active)
            .count()
    }

    fn cycle(&self) -> Space {
        println!("cycle in\n{}", self);
        // build a space 2 larger in each dim:
        let mut xs = Space::new(self.dim_x + 2, self.dim_y + 2, self.dim_z + 2);
        // place old data in new space:
        for x in 0..self.dim_x {
            for y in 0..self.dim_y {
                for z in 0..self.dim_z {
                    xs.set_xyz(x + 1, y + 1, z + 1, self.get_xyz(x, y, z));
                }
            }
        }
        // now run the Conway algorithm from xs to a new space:
        let mut res = Space::new(xs.dim_x, xs.dim_y, xs.dim_z);
        for x in 0..xs.dim_x {
            for y in 0..xs.dim_y {
                for z in 0..xs.dim_z {
                    let active = xs.active_neighbors(x, y, z);
                    match xs.get_xyz(x, y, z) {
                        State::Active => {
                            if active == 2 || active == 3 {
                                res.set_xyz(x, y, z, State::Active)
                            } else {
                                res.set_xyz(x, y, z, State::Inactive)
                            }
                        }
                        State::Inactive => {
                            if active == 3 {
                                res.set_xyz(x, y, z, State::Active)
                            } else {
                                res.set_xyz(x, y, z, State::Inactive)
                            }
                        }
                    }
                }
            }
        }
        println!("cycle out\n{}", res);
        res
    }

    fn cycles(&self, n: usize) -> Space {
        let mut s = self.clone();
        for _ in 0..n {
            let xs = s.cycle();
            s = xs;
        }
        s
    }

    fn xyz(&self, x: usize, y: usize, z: usize) -> usize {
        x + (self.dim_x * y) + (self.dim_x * self.dim_y * z)
    }

    fn get_xyz(&self, x: usize, y: usize, z: usize) -> State {
        self.data[self.xyz(x, y, z)]
    }

    fn set_xyz(&mut self, x: usize, y: usize, z: usize, v: State) {
        let idx = self.xyz(x, y, z);
        self.data[idx] = v;
    }
}

impl PartialEq for Space {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl fmt::Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = String::with_capacity(self.dim_y * (self.dim_x + 1));
        for z in 0..self.dim_z {
            writeln!(&mut res, "z={}", z).unwrap();
            for y in 0..self.dim_y {
                for x in 0..self.dim_x {
                    match self.get_xyz(x, y, z) {
                        State::Inactive => write!(&mut res, ".").unwrap(),
                        State::Active => write!(&mut res, "#").unwrap(),
                    }
                }
                writeln!(&mut res).unwrap();
            }
            writeln!(&mut res).unwrap();
        }
        write!(f, "{}", res)
    }
}

// A Hyperspace is a 4d map
#[derive(Clone, Debug)]
struct Hyperspace {
    dim_x: usize,
    dim_y: usize,
    dim_z: usize,
    dim_w: usize,
    data: Vec<State>,
}

impl Hyperspace {
    fn new(dim_x: usize, dim_y: usize, dim_z: usize, dim_w: usize) -> Hyperspace {
        let data = vec![State::Inactive; dim_x * dim_y * dim_z * dim_w];
        Hyperspace {
            dim_x,
            dim_y,
            dim_z,
            dim_w,
            data,
        }
    }

    fn parse(raw: &str) -> Hyperspace {
        let dim_x = raw.lines().count();
        let dim_y = raw.lines().next().unwrap().len();
        let mut s = Hyperspace::new(dim_x, dim_y, 1, 1);
        for (y, line) in raw.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '.' => s.set_xyzw(x, y, 0, 0, State::Inactive),
                    '#' => s.set_xyzw(x, y, 0, 0, State::Active),
                    _ => panic!("bad input"),
                }
            }
        }
        s
    }

    fn active_count(&self) -> i64 {
        self.data.iter().filter(|&&s| s == State::Active).count() as i64
    }

    fn neighbors(
        &self,
        x: usize,
        y: usize,
        z: usize,
        w: usize,
    ) -> Vec<(usize, usize, usize, usize)> {
        let mut result = Vec::new();
        let xmin = if x == 0 { 0 } else { x - 1 };
        let ymin = if y == 0 { 0 } else { y - 1 };
        let zmin = if z == 0 { 0 } else { z - 1 };
        let wmin = if w == 0 { 0 } else { w - 1 };
        for i in xmin..=cmp::min(x + 1, self.dim_x - 1) {
            for j in ymin..=cmp::min(y + 1, self.dim_y - 1) {
                for k in zmin..=cmp::min(z + 1, self.dim_z - 1) {
                    for l in wmin..=cmp::min(w + 1, self.dim_w - 1) {
                        if i != x || j != y || k != z || l != w {
                            result.push((i, j, k, l));
                        }
                    }
                }
            }
        }
        result
    }

    fn active_neighbors(&self, x: usize, y: usize, z: usize, w: usize) -> usize {
        self.neighbors(x, y, z, w)
            .iter()
            .filter(|(i, j, k, l)| self.get_xyzw(*i, *j, *k, *l) == State::Active)
            .count()
    }

    fn cycle(&self) -> Hyperspace {
        println!("cycle in\n{}", self);
        // build a space 2 larger in each dim:
        let mut xs = Hyperspace::new(
            self.dim_x + 2,
            self.dim_y + 2,
            self.dim_z + 2,
            self.dim_w + 2,
        );
        // place old data in new space:
        for x in 0..self.dim_x {
            for y in 0..self.dim_y {
                for z in 0..self.dim_z {
                    for w in 0..self.dim_w {
                        xs.set_xyzw(x + 1, y + 1, z + 1, w + 1, self.get_xyzw(x, y, z, w));
                    }
                }
            }
        }
        // now run the Conway algorithm from xs to a new space:
        let mut res = Hyperspace::new(xs.dim_x, xs.dim_y, xs.dim_z, xs.dim_w);
        for x in 0..xs.dim_x {
            for y in 0..xs.dim_y {
                for z in 0..xs.dim_z {
                    for w in 0..xs.dim_w {
                        let active = xs.active_neighbors(x, y, z, w);
                        match xs.get_xyzw(x, y, z, w) {
                            State::Active => {
                                if active == 2 || active == 3 {
                                    res.set_xyzw(x, y, z, w, State::Active)
                                } else {
                                    res.set_xyzw(x, y, z, w, State::Inactive)
                                }
                            }
                            State::Inactive => {
                                if active == 3 {
                                    res.set_xyzw(x, y, z, w, State::Active)
                                } else {
                                    res.set_xyzw(x, y, z, w, State::Inactive)
                                }
                            }
                        }
                    }
                }
            }
        }
        println!("cycle out\n{}", res);
        res
    }

    fn cycles(&self, n: usize) -> Hyperspace {
        let mut s = self.clone();
        for _ in 0..n {
            let xs = s.cycle();
            s = xs;
        }
        s
    }

    fn xyzw(&self, x: usize, y: usize, z: usize, w: usize) -> usize {
        x + (self.dim_x * y)
            + (self.dim_x * self.dim_y * z)
            + (self.dim_x * self.dim_y * self.dim_z * w)
    }

    fn get_xyzw(&self, x: usize, y: usize, z: usize, w: usize) -> State {
        self.data[self.xyzw(x, y, z, w)]
    }

    fn set_xyzw(&mut self, x: usize, y: usize, z: usize, w: usize, v: State) {
        let idx = self.xyzw(x, y, z, w);
        self.data[idx] = v;
    }
}

impl PartialEq for Hyperspace {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl fmt::Display for Hyperspace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = String::with_capacity(self.dim_y * (self.dim_x + 1));
        for w in 0..self.dim_w {
            for z in 0..self.dim_z {
                writeln!(&mut res, "z={}, w={}", z, w).unwrap();
                for y in 0..self.dim_y {
                    for x in 0..self.dim_x {
                        match self.get_xyzw(x, y, z, w) {
                            State::Inactive => write!(&mut res, ".").unwrap(),
                            State::Active => write!(&mut res, "#").unwrap(),
                        }
                    }
                    writeln!(&mut res).unwrap();
                }
                writeln!(&mut res).unwrap();
            }
        }
        write!(f, "{}", res)
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("err reading the file");
    let s = Space::parse(&contents);
    println!("{}", s);
    println!("part 1 = {}", s.cycles(6).active_count());
    let hs = Hyperspace::parse(&contents);
    println!("{}", hs);
    println!("part 2 = {}", hs.cycles(6).active_count());
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = ".#.
..#
###";

    #[test]
    fn test_part1() {
        let s = Space::parse(DATA);
        eprintln!("{}", s);
        assert_eq!(s.get_xyz(0, 0, 0), State::Inactive);
        assert_eq!(s.get_xyz(1, 0, 0), State::Active);
        assert_eq!(s.get_xyz(2, 1, 0), State::Active);
        assert_eq!(s.cycles(6).active_count(), 112);
    }

    #[test]
    fn test_part2() {
        let s = Hyperspace::parse(DATA);
        eprintln!("{}", s);
        assert_eq!(s.get_xyzw(0, 0, 0, 0), State::Inactive);
        assert_eq!(s.get_xyzw(1, 0, 0, 0), State::Active);
        assert_eq!(s.get_xyzw(2, 1, 0, 0), State::Active);
        assert_eq!(s.cycles(6).active_count(), 848);
    }
}
