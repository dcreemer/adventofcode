// solution to
// https://adventofcode.com/2020/day/7

use lazy_static::lazy_static;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::depth_first_search;
use petgraph::visit::DfsEvent;
use regex::Regex;
use std::fs;

// We keep two directed graphs, one representing "is-contained-by" relationships,
// and one with "contains" relationships. The nodes are named, and the edges contain a count.
type MyGraph = Graph<String, i32>;

fn parse_graph(data: &str) -> (MyGraph, MyGraph) {
    let mut is_contained_by = Graph::<String, i32>::new();
    let mut contains = Graph::<String, i32>::new();
    data.lines()
        .for_each(|l| parse_edge(&mut is_contained_by, &mut contains, l));
    (is_contained_by, contains)
}

fn find_node(g: &MyGraph, node: &str) -> Option<NodeIndex> {
    g.node_indices().find(|i| g[*i] == node)
}

fn find_or_add_node(g: &mut MyGraph, node: &str) -> NodeIndex {
    if let Some(index) = find_node(g, node) {
        return index;
    }
    g.add_node(node.to_string())
}

fn parse_edge(is_contained_by: &mut MyGraph, contains: &mut MyGraph, line: &str) {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"\s*(\d)+ ([[:alpha:]]+ [[:alpha:]]+) bags?\.?").unwrap();
    }
    let pieces: Vec<&str> = line.split(" bags contain ").collect();
    let outer_a = find_or_add_node(is_contained_by, pieces[0]);
    let outer_b = find_or_add_node(contains, pieces[0]);
    for e in pieces[1].split(", ") {
        if let Some(cap) = RE.captures(e) {
            let ct = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let inner = cap.get(2).unwrap().as_str();
            let inner_a = find_or_add_node(is_contained_by, inner);
            let inner_b = find_or_add_node(contains, inner);
            // inner is-contained-by outer
            is_contained_by.add_edge(inner_a, outer_a, ct);
            // outer contains inner
            contains.add_edge(outer_b, inner_b, ct);
        }
    }
}

fn count_contains(g: &MyGraph, from: &str) -> i32 {
    let start = find_node(g, from).unwrap();
    let mut count = 0;
    depth_first_search(&g, Some(start), |event| {
        if let DfsEvent::TreeEdge(_u, _v) = event {
            // println!("{:#?} is-contained-by {:#?}", g[u], g[v]);
            count += 1;
        }
    });
    count
}

fn sum_inner(g: &MyGraph, current: NodeIndex) -> i32 {
    // println!("SUM {:#?}", g[current]);
    if g.neighbors(current).count() > 0 {
        let mut s = 1;
        for n in g.neighbors(current) {
            let e = g.find_edge(current, n).unwrap();
            s += g[e] * sum_inner(g, n)
        }
        s
    } else {
        1
    }
}

fn sum_all(g: &MyGraph, from: &str) -> i32 {
    sum_inner(g, find_node(g, from).unwrap()) - 1
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("err reading the file");
    let (is_contained_by, contains) = parse_graph(&contents);
    println!("part 1 = {}", count_contains(&is_contained_by, "shiny gold"));
    println!("part 2 = {}", sum_all(&contains, "shiny gold"));
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    const DATA2: &str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
";

    #[test]
    fn test_parse() {
        let (g1, g2) = parse_graph(DATA);
        assert_eq!(g1.node_count(), 9);
        assert_eq!(g1.edge_count(), 13);
        assert_eq!(g2.node_count(), 9);
        assert_eq!(g2.edge_count(), 13);
    }

    #[test]
    fn test_walk_1() {
        let (g1, _) = parse_graph(DATA);
        assert_eq!(count_contains(&g1, "shiny gold"), 4);
    }

    #[test]
    fn test_walk_2() {
        let (_, g2) = parse_graph(DATA);
        assert_eq!(sum_all(&g2, "shiny gold"), 32);
        let (_, g2) = parse_graph(DATA2);
        assert_eq!(sum_all(&g2, "shiny gold"), 126);
    }
}
