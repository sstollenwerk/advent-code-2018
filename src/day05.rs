use crate::lib::to_filename;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

use itertools::Itertools;

type Num = u32;

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
    let mut i = vals.len() - 2;
    while true {
        let (a, b) = (vals[i], vals[i + 1]);
        if a.eq_ignore_ascii_case(&b) && a != b {
            vals.remove(i);
            vals.remove(i);
        }
        if i == 0 {
            break;
        }
        // is usize, dealing with converting to/from int is a pain
        i -= 1;
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

pub fn part2() -> Num {
    let vals = get_data();
    todo!();
}
