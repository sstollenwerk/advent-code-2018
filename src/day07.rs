use crate::lib::to_filename;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

use itertools::Itertools;

type Node = char;
type Edge = (Node, Node);
type Graph = HashMap<Node, HashSet<Node>>;

fn get_data() -> Vec<Edge> {
    fs::read_to_string(to_filename(07))
        .expect("Could not read file")
        .lines()
        .map(|s| read_row(s))
        .collect()
}

fn read_row(row: &str) -> Edge {
    let r = row.split(" ").collect::<Vec<_>>();
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
        res.push((check.clone()));

        edges.retain(|(a, _)| a != check);
        (standard, reversed) = make_graphs(&edges);
    }

    let a = edges_.iter().map(|(_, b)| *b).collect::<HashSet<_>>();
    let seen = res.clone().into_iter().collect::<HashSet<_>>();

    let remaining_ = a.difference(&(seen));

    let mut remaining = remaining_.map(|x| *x).collect::<Vec<_>>();

    res.append(&mut remaining);
    res
}

fn make_graphs(edges: &Vec<Edge>) -> (Graph, Graph) {
    let backwards = edges.iter().map(|(a, b)| (*b, *a)).collect();
    let standard = make_graph(&edges);
    let reversed = make_graph(&backwards);
    (standard, reversed)
}

fn make_graph(edges: &Vec<Edge>) -> Graph {
    let mut res = HashMap::new();
    for (a, b) in edges.iter() {
        res.entry(*a).or_insert(HashSet::new()).insert(*b);
    }
    res
}

pub fn part1() -> String {
    let vals = get_data();

    let r = topological_sort(&vals);

    r.iter().cloned().collect::<String>()
}

pub fn part2() -> usize {
    let vals = get_data();
    todo!();
}
