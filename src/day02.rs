use crate::lib::to_filename;

use std::collections::HashSet;
use std::fs;

use counter::Counter;

type Num = u32;

fn get_data() -> Vec<String> {
    fs::read_to_string(to_filename(02))
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

pub fn part1() -> usize {
    let vals = get_data();

    vals.iter().filter( |x| appears(2, &x)  ).count() * vals.iter().filter( |x| appears(3, &x)  ).count()

    //todo!();
}

pub fn part2() -> Num {
    todo!();
}
