use lib::Rule;
use std::{
    cell::Cell,
    collections::{HashMap, HashSet, VecDeque},
    io,
};

#[derive(Debug, Default)]
struct Node {
    visited: Cell<bool>,
    parents: HashSet<String>,
}

type Graph = HashMap<String, Node>;

fn inverse_graph(rules: &[Rule]) -> Graph {
    let mut nodes = Graph::default();
    for rule in rules {
        nodes.entry(rule.color.clone()).or_default();
        for (_, child) in &rule.nested {
            nodes
                .entry(child.clone())
                .or_default()
                .parents
                .insert(rule.color.clone());
        }
    }
    nodes
}

fn count_reachable(graph: &Graph, start: &str) -> usize {
    let mut count = 0;
    let mut deque = VecDeque::default();

    let start = graph.get(start).unwrap();
    deque.extend(start.parents.iter().map(String::as_str));
    start.visited.set(true);

    while let Some(node) = deque.pop_front() {
        let node = graph.get(node).unwrap();
        if !node.visited.get() {
            node.visited.set(true);
            count += 1;
            deque.extend(node.parents.iter().map(String::as_str));
        }
    }
    count
}

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let data = lib::read_rules(stdin);
    let graph = inverse_graph(&data);

    println!("{}", count_reachable(&graph, "shiny gold"));
}
