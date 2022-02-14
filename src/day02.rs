use crate::lib::to_filename;

use std::collections::HashSet;
use std::fs;

use counter::Counter;
use itertools::Itertools;

type Num = u32;

fn get_data() -> Vec<String> {
    fs::read_to_string(to_filename(2))
        .expect("Could not read file")
        .lines()
        .map(|s| s.to_string())
        .collect()
}

fn counts(vals: &String) -> HashSet<Num> {
    vals.chars()
        .collect::<Counter<_>>()
        .values()
        .map(|x| *x as Num)
        .collect::<HashSet<_>>()
        .to_owned()
}

fn appears(n: Num, vals: &String) -> bool {
    counts(&vals).contains(&n)
}

fn similarity(a: &String, b: &String) -> Option<String> {
    let similars: String = a
        .chars()
        .zip(b.chars())
        .filter(|(x, y)| x == y)
        .map(|(x, y)| x)
        .collect();
    if similars.len() + 1 == a.len() {
        Some(similars)
    } else {
        None
    }
}

pub fn part1() -> usize {
    let vals = get_data();

    vals.iter().filter(|x| appears(2, &x)).count() * vals.iter().filter(|x| appears(3, &x)).count()
}

pub fn part2() -> String {
    let vals = get_data();
    vals.iter()
        .combinations(2)
        .filter_map(|g| similarity(g[0], g[1]))
        .next()
        .unwrap()
}
