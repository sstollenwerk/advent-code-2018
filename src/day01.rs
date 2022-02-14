use crate::lib::to_filename;

use std::collections::HashSet;
use std::fs;

type Num = i32;

fn get_data() -> Vec<Num> {
    fs::read_to_string(to_filename(1))
        .expect("Could not read file")
        .lines()
        .map(|s| read_row(s))
        .collect()
}

fn read_row(row: &str) -> Num {
    row.parse::<Num>().unwrap()
}

pub fn part1() -> Num {
    let vals = get_data();

    vals.into_iter().sum()
}

pub fn part2() -> Num {
    let mut seen = HashSet::new();

    let mut res = 0;

    for n in get_data().into_iter().cycle() {
        res += n;
        if !seen.insert(res) {
            return res;
        }
    }

    unreachable!()
}
