use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug, PartialEq)]
struct Graph {
    edges: HashMap<char, HashSet<char>>,
    letters: HashSet<char>,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            edges: HashMap::new(),
            letters: HashSet::new(),
        }
    }

    fn add(&mut self, a: char, b: char) {
        let tl = self.edges.entry(b).or_insert_with(HashSet::new);
        tl.insert(a);
        self.letters.insert(a);
        self.letters.insert(b);
    }

    fn remove(&mut self, c: char) {
        for lt in self.edges.values_mut() {
            lt.remove(&c);
        }
        self.edges.remove(&c);
    }

    fn finalize(&mut self) {
        for l in self.letters.iter() {
            self.edges.entry(*l).or_insert_with(HashSet::new);
        }
    }

    fn len(&self) -> usize {
        self.edges.len()
    }

    fn frontier(&self) -> Vec<char> {
        let mut r: Vec<char> = self
            .edges
            .iter()
            .filter_map(|(&f, tl)| if tl.is_empty() { Some(f) } else { None })
            .collect();
        r.sort();
        r
    }

    fn walk(self: &mut Graph) -> String {
        let mut r: Vec<char> = vec![];
        while self.len() > 0 {
            let v = self.frontier();
            r.push(v[0]);
            self.remove(v[0]);
        }
        r.into_iter().collect()
    }
}

impl From<&str> for Graph {
    fn from(s: &str) -> Self {
        let mut g = Graph::new();
        for line in s.lines() {
            let mut i = line.chars();
            g.add(i.nth(5).unwrap(), i.nth(30).unwrap());
        }
        g.finalize();
        g
    }
}

fn main() {
    println!("AoC 2018 Seven");
    let content: &str = &fs::read_to_string("input.txt").expect("err reading the file");
    let mut g = Graph::from(content);
    println!("Part one - steps order {}", g.walk());
    println!("Part two: TBD");
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_graph() -> Graph {
        let mut g = Graph::new();
        g.add('C', 'A');
        g.add('C', 'F');
        g.add('A', 'B');
        g.add('A', 'D');
        g.add('B', 'E');
        g.add('D', 'E');
        g.add('F', 'E');
        g.finalize();
        g
    }

    #[test]
    fn test_parse_graph() {
        let mut g = Graph::new();
        g.add('C', 'A');
        g.finalize();
        assert_eq!(
            Graph::from("Step C must be finished before step A can begin."),
            g
        );
    }

    #[test]
    fn test_frontier() {
        assert_eq!(test_graph().frontier(), vec!['C']);
    }

    #[test]
    fn test_remove() {
        let mut g = test_graph();
        g.remove('C');
        assert_eq!(g.frontier(), vec!['A', 'F']);
    }

    #[test]
    fn test_walk() {
        let mut g = test_graph();
        assert_eq!(g.walk(), "CABDFE");
    }
}
