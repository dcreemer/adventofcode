use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug, PartialEq)]
struct Graph {
    // edges maps from a node to a set of other nodes
    edges: HashMap<char, HashSet<char>>,
    nodes: HashSet<char>,
}

/// Graph maintain a set of nodes and edges.
impl Graph {
    fn new() -> Graph {
        Graph {
            edges: HashMap::new(),
            nodes: HashSet::new(),
        }
    }

    /// add an edge (and implicitly the nodes)
    fn add(&mut self, a: char, b: char) {
        self.edges.entry(a).or_insert_with(HashSet::new);
        let tl = self.edges.entry(b).or_insert_with(HashSet::new);
        tl.insert(a);
        self.nodes.insert(a);
        self.nodes.insert(b);
    }

    /// remove a node (and all impacted edges)
    fn remove(&mut self, c: char) {
        for lt in self.edges.values_mut() {
            lt.remove(&c);
        }
        self.edges.remove(&c);
        self.nodes.remove(&c);
    }

    fn len(&self) -> usize {
        self.edges.len()
    }

    /// returns the set of all nodes that have no inbound edges
    /// sorted alphabetically
    fn frontier(&self) -> Vec<char> {
        let mut r: Vec<char> = self
            .edges
            .iter()
            .filter_map(|(&f, tl)| if tl.is_empty() { Some(f) } else { None })
            .collect();
        r.sort();
        r
    }

    /// Peek at the next piece of work from the graph, exlucing those
    /// chars in the given set
    fn peek_work(self: &Graph, excl: &HashSet<char>) -> Option<char> {
        let v = self.frontier();
        if v.len() > 0 {
            for c in v.iter() {
                if !excl.contains(&c) {
                    return Some(*c);
                }
            }
        }
        None
    }

    /// Pop at the next piece of work from the graph
    fn pop_work(self: &mut Graph) -> Option<char> {
        let v = self.frontier();
        if v.len() > 0 {
            self.remove(v[0]);
            Some(v[0])
        } else {
            None
        }
    }
}

fn solve_1(g: &mut Graph) -> String {
    let mut r: Vec<char> = vec![];
    while let Some(c) = g.pop_work() {
        r.push(c);
    }
    r.into_iter().collect()
}

fn ordinal(c: char) -> u32 {
    c.to_digit(36).unwrap() - 10
}

fn solve_2(g: &mut Graph, wcount: usize, base: u32) -> (String, u32) {
    let mut result: Vec<char> = vec![];
    let mut workers: Vec<(char, u32)> = vec![(' ', 0); wcount];
    let mut active: HashSet<char> = HashSet::new();
    let mut time: u32 = 0;
    loop {
        for i in 0..wcount {
            let (c, tick) = workers[i];
            if tick == 0 {
                // previous work is done!
                if c >= 'A' && c <= 'Z' {
                    g.remove(c);
                    active.remove(&c);
                    result.push(c);
                    workers[i] = (' ', 0);
                }
                // ready for more work!
                if let Some(c) = g.peek_work(&active) {
                    // each worker works for ordinal(c)+base ticks
                    workers[i] = (c, ordinal(c) + base);
                    active.insert(c);
                }
            } else {
                // count down:
                workers[i] = (c, tick - 1);
            }
        }
        // are we done?
        if g.len() == 0 && active.len() == 0 {
            break;
        }
        // nope; the clock keeps ticking
        time += 1;
    }
    (result.into_iter().collect(), time)
}

/// Construct a graph from a string of lines in the form
/// "Step G must be finished before step M can begin."
impl From<&str> for Graph {
    fn from(s: &str) -> Self {
        let mut g = Graph::new();
        for line in s.lines() {
            let mut i = line.chars();
            g.add(i.nth(5).unwrap(), i.nth(30).unwrap());
        }
        g
    }
}

fn main() {
    println!("AoC 2018 Seven");
    let content: &str = &fs::read_to_string("input.txt").expect("err reading the file");
    let mut g = Graph::from(content);
    println!("Part one - steps order {}", solve_1(&mut g));
    let mut g = Graph::from(content);
    let (_r, t) = solve_2(&mut g, 5, 60);
    println!("Part two - time to solve = {}", t);
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
        g
    }

    #[test]
    fn test_parse_graph() {
        let mut g = Graph::new();
        g.add('C', 'A');
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
    fn test_solve_1() {
        let mut g = test_graph();
        assert_eq!(solve_1(&mut g), "CABDFE");
    }

    #[test]
    fn test_ordinal() {
        assert_eq!(ordinal('A'), 0);
        assert_eq!(ordinal('Z'), 25);
    }

    #[test]
    fn test_solve_2() {
        let mut g = test_graph();
        let (r, s) = solve_2(&mut g, 2, 0);
        assert_eq!(r, "CABFDE");
        assert_eq!(s, 15);
    }
}
