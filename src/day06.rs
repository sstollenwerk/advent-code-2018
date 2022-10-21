use std::collections::HashMap;
use std::collections::HashSet;

use itertools::Itertools;
use num_complex::Complex;

type Num = i32;

type Position = Complex<Num>;

fn parse(s: &str) -> Vec<Position> {
    s.lines().map(read_row).collect()
}

fn read_row(row: &str) -> Position {
    let parts: Vec<Num> = row
        .split(',')
        .map(|n| n.trim().parse::<Num>().unwrap())
        .collect();
    Position::new(parts[0], parts[1])
}

fn corners(xs: &[Position]) -> (Position, Position) {
    let a = xs.iter().map(|c| c.re).min().unwrap();
    let b = xs.iter().map(|c| c.im).min().unwrap();

    let c = xs.iter().map(|c| c.re).max().unwrap();
    let d = xs.iter().map(|c| c.im).max().unwrap();

    (Position::new(a, b), Position::new(c, d))
}

fn dists(p: &Position, xs: &[Position]) -> Vec<Num> {
    xs.iter().map(|c| (p - c).l1_norm()).collect()
}

fn nearests(
    to_check: &Vec<Position>,
    locations: &[Position],
) -> HashMap<Position, HashSet<Position>> {
    let mut closests: HashMap<Position, HashSet<Position>> = HashMap::new();

    for p in to_check {
        let distances = dists(p, locations);
        if (distances.iter().min_set().len() == 1) {
            let closest = distances.iter().enumerate().min_by_key(|t| t.1).unwrap().0;
            closests.entry(locations[closest]).or_default().insert(*p);
        }
    }

    closests
}

fn finites(locations: &[Position]) -> HashMap<Position, HashSet<Position>> {
    let a = 5;
    let b = 10;

    let (top, down) = corners(locations);

    let ax = (top.re - a..down.re + a)
        .cartesian_product(top.im - a..down.im + a)
        .map(|(x, y)| Position::new(x, y))
        .collect();
    let bx = (top.re - b..down.re + b)
        .cartesian_product(top.im - b..down.im + b)
        .map(|(x, y)| Position::new(x, y))
        .collect();

    let mut posses = nearests(&ax, locations);
    let crossreference = nearests(&bx, locations);

    posses.retain(|k, v| &crossreference[k] == v);

    posses
}

pub fn part1(s: &str) -> usize {
    let vals = parse(s);

    let locs = finites(&vals);

    for (k, v) in locs.iter() {
        println!("{:?}", &(k, v.len()));
    }

    locs.values().map(|r| r.len()).max().unwrap()
}

pub fn part2(s: &str) -> Num {
    todo!();
}
