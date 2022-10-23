

use crate::helper::{Num};


fn parse(s: &str) -> Vec<Num> {
    s.lines().map(read_row).collect()
}

fn read_row(row: &str) -> Num {
    todo!();
}



pub fn part1(s: &str) -> Num {
    let thing = parse(s);

    todo!();
}

pub fn part2(s: &str) -> Num {
    todo!();

}
