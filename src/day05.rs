use crate::lib::to_filename;

use std::collections::HashSet;
use std::fs;

use itertools::Itertools;


fn get_data() -> Vec<char> {
    fs::read_to_string(to_filename(05))
        .expect("Could not read file")
        .lines()
        .next()
        .unwrap()
        .chars()
        .collect()
}

fn reduce(vals_: &Vec<char>) -> Vec<char> {
    let mut vals = vals_.clone();

    let mut i = 0 ;
    while i < vals.len() - 1 {
        let (a, b) = (vals[i], vals[i + 1]);
        if a.eq_ignore_ascii_case(&b) && a != b {
            vals.remove(i);
            vals.remove(i);
        }
        i += 1;
    }
    if &vals == vals_ {
        vals
    } else {
        reduce(&vals)
    }
}

pub fn part1() -> usize {
    let vals = get_data();

    reduce(&vals).len()
}

fn make_posses(vals: &Vec<char>) -> Vec<Vec<char>> {
    let chars: HashSet<char> = vals.iter().map(|c| c.to_ascii_lowercase()).collect();
    let mut res: Vec<Vec<char>> = Vec::new();
    for c in chars.iter() {
        let p = vals
            .iter()
            .filter(|x| !(c.eq_ignore_ascii_case(x))).map(|x| *x)
            .collect();
        res.push(p);
    }
    res
}

pub fn part2() -> usize {
    let vals = get_data();
    let posses = make_posses(&vals);
    posses.iter().map(reduce).map(|x| x.len()).min().unwrap()
}
