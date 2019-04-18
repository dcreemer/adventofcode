use std::collections::HashMap;
use std::fs;

/// A Tree!

#[derive(Debug, PartialEq)]
struct Node {
    idx: usize,
    // each child is an index into the tree's data Vec
    children: Vec<usize>,
    meta: Vec<usize>,
}

impl Node {
    fn new(idx: usize) -> Node {
        Node {
            idx: idx,
            children: vec![],
            meta: vec![],
        }
    }

    fn sum_metadata(&self) -> usize {
        self.meta.iter().sum()
    }
}

#[derive(Debug, PartialEq)]
struct Tree {
    // original data
    data: Vec<usize>,
    // maps from node index in data vec to Node
    nodes: HashMap<usize, Node>,
}

impl Tree {
    fn parse(s: &str) -> Tree {
        let mut t = Tree {
            data: s.split_whitespace().map(|i| i.parse().unwrap()).collect(),
            nodes: HashMap::new(),
        };
        let mut idx: usize = 0;
        t.parse_node(&mut idx);
        t
    }

    fn parse_node(&mut self, i: &mut usize) -> usize {
        let idx = *i;
        let mut n = Node::new(idx);
        let child_count = self.data[*i];
        *i += 1;
        let meta_count = self.data[*i];
        *i += 1;
        for _ in 0..child_count {
            n.children.push(self.parse_node(i));
        }
        for _ in 0..meta_count {
            n.meta.push(self.data[*i]);
            *i += 1;
        }
        self.nodes.insert(idx, n);
        idx
    }

    fn sum_metadata(&self) -> usize {
        self.nodes.values().fold(0, |acc, n| acc + n.sum_metadata())
    }

    fn node_value(&self, idx: usize) -> usize {
        let n = self.nodes.get(&idx).unwrap();
        if n.children.len() == 0 {
            n.sum_metadata()
        } else {
            n.meta.iter().fold(0, |acc, ci| {
                if let Some(cni) = n.children.get(ci - 1) {
                    acc + self.node_value(*cni)
                } else {
                    acc
                }
            })
        }
    }
}

fn main() {
    println!("AoC 2018 Eight");
    let content: &str = &fs::read_to_string("input.txt").expect("err reading the file");
    let t = Tree::parse(content);
    println!("Part one - sum of meta = {}", t.sum_metadata());
    println!("Part two - value of root node = {}", t.node_value(0));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let t: Tree = Tree::parse("0 1 2");
        assert_eq!(3, t.data.len());
    }

    #[test]
    fn test_solve_1() {
        let t: Tree = Tree::parse("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2");
        assert_eq!(138, t.sum_metadata());
    }

    #[test]
    fn test_node_value() {
        let t: Tree = Tree::parse("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2");
        assert_eq!(33, t.node_value(2));
        assert_eq!(99, t.node_value(9));
        assert_eq!(0, t.node_value(7));
        assert_eq!(66, t.node_value(0));
    }
}
