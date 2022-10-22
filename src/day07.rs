use std::collections::HashMap;
use std::collections::HashSet;

use itertools::Itertools;

type Node = char;
type Edge = (Node, Node);
type Graph = HashMap<Node, HashSet<Node>>;

fn parse(s: &str) -> Vec<Edge> {
    s.lines().map(read_row).collect()
}

fn read_row(row: &str) -> Edge {
    let r = row.split(' ').collect::<Vec<_>>();
    (r[1].chars().next().unwrap(), r[7].chars().next().unwrap())
}

fn topological_sort(edges_: &Vec<Edge>) -> Vec<Node> {
    // using Kahn's algorithm
    // https://en.wikipedia.org/wiki/Topological_sorting#Kahn's_algorithm

    let mut edges = edges_.to_owned();

    //let backwards = edges.iter().map(|(a, b)| (*b, *a)).collect();
    //let mut standard = make_graph(&edges);
    //let mut reversed = make_graph(&backwards);
    let (mut standard, mut reversed) = make_graphs(&edges);
    let mut res = Vec::new();

    // not sure if should be reversed, standard or both?

    while !standard.is_empty() {
        let check = *(standard
            .keys()
            .collect::<HashSet<_>>()
            .difference(&reversed.keys().collect::<HashSet<_>>())
            .sorted()
            .next()
            .unwrap());
        res.push(*check);

        edges.retain(|(a, _)| a != check);
        (standard, reversed) = make_graphs(&edges);
    }

    let a = edges_.iter().map(|(_, b)| *b).collect::<HashSet<_>>();
    let seen = res.clone().into_iter().collect::<HashSet<_>>();

    let remaining_ = a.difference(&(seen));

    let mut remaining = remaining_.copied().collect::<Vec<_>>();

    res.append(&mut remaining);
    res
}

fn time(n: Node) -> u32 {
    60 + n as u32 - ('A' as u32) + 1
}

fn schedule(edges: &Vec<Edge>, elves: u32) -> u32 {
    let workers = elves + 1;
    let mut t: u32 = 0;

    let mut todo: HashSet<Node> = edges.iter().flat_map(|(a, b)| [a, b]).copied().collect();

    let mut doing: HashMap<Node, u32> = HashMap::new();

    let (standard, _) = make_graphs(edges);

    while !todo.is_empty() {
        let to_add = (workers as usize) - (doing.len());

        let cant_do: HashSet<Node> = todo
            .iter()
            .filter_map(|c| standard.get(c))
            .fold(HashSet::new(), |acc, x| &acc | x);

        let check: Vec<Node> = (&(&todo - &cant_do)
            - &(doing.keys().copied().collect::<HashSet<Node>>()))
            .into_iter()
            .sorted()
            .take(to_add)
            .collect::<Vec<Node>>();

        for k in check {
            doing.insert(k, time(k));
        }

        let items: Vec<_> = doing.keys().copied().collect();

        for k in items {
            doing.entry(k).and_modify(|c| *c -= 1);
            if doing[&k] == 0 {
                doing.remove(&k);
                todo.remove(&k);
            }
        }

        t += 1;
    }

    t
}

fn make_graphs(edges: &Vec<Edge>) -> (Graph, Graph) {
    let backwards = edges.iter().map(|(a, b)| (*b, *a)).collect();
    let standard = make_graph(edges);
    let reversed = make_graph(&backwards);
    (standard, reversed)
}

fn make_graph(edges: &Vec<Edge>) -> Graph {
    let mut res: HashMap<Node, HashSet<Node>> = HashMap::new();
    for (a, b) in edges.iter() {
        res.entry(*a).or_default().insert(*b);
    }
    res
}

pub fn part1(s: &str) -> String {
    let vals = parse(s);

    let r = topological_sort(&vals);

    r.iter().cloned().collect::<String>()
}

pub fn part2(s: &str) -> u32 {
    let vals = parse(s);
    schedule(&vals, 4)
}
