use crate::lib::to_filename;

use std::collections::HashSet;
use std::fs;

use counter::Counter;
use num_complex::Complex;

type Num = u32;

type Position = Complex<i32>;
type Size = Complex<i32>;

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
struct Claim {
    id: Num,
    position: Position,
    size: Size,
}

impl Claim {
    fn new(id: Num, position: Position, size: Size) -> Claim {
        Claim { id, position, size }
    }

    fn positions(&self) -> Vec<Position> {
        let mut res: Vec<Position> = Vec::new();
        for i in (0..self.size.re) {
            for j in (0..self.size.im) {
                res.push(self.position + Complex::new(i, j));
            }
        }
        res
    }
}

fn get_data() -> Vec<Claim> {
    fs::read_to_string(to_filename(03))
        .expect("Could not read file")
        .lines()
        .map(|s| read_row(s))
        .collect()
}

fn read_row(row: &str) -> Claim {
    let row = row
        .replace("#", "")
        .replace("@", "")
        .replace(":", "")
        .replace("x", ",");
    let parts: Vec<String> = row.split(' ').map(String::from).collect();
    assert_eq!(parts.len(), 4);
    let id = (parts[0]).parse::<Num>().unwrap();
    Claim::new(id, parse_section(&parts[2]), parse_section(&parts[3]))
}

fn parse_section(section: &str) -> Complex<i32> {
    let parts: Vec<i32> = section
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    Complex::new(parts[0], parts[1])
}

fn overlap(claims: &Vec<Claim>) -> usize {
    let mut seen: HashSet<Position> = HashSet::new();
    let mut dupes: HashSet<Position> = HashSet::new();
    for i in claims {
        for p in i.positions() {
            let within = seen.insert(p);
            if !within {
                dupes.insert(p);
            }
        }
    }
    dupes.len()
}

pub fn part1() -> usize {
    let vals = get_data();

    overlap(&vals)
}

pub fn part2() -> usize {
    let vals = get_data();
    todo!();
}
