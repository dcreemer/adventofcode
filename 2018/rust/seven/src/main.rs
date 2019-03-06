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

    /// destructively walk the graph and return the nodes
    /// as a String
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
    fn test_walk() {
        let mut g = test_graph();
        assert_eq!(g.walk(), "CABDFE");
    }
}
