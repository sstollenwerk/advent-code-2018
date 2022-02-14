use crate::lib::to_filename;

use std::fs;
use std::collections::HashSet;

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
    todo!()
}
