use std::collections::HashSet;

use num_complex::Complex;

use crate::helper::{s_display, to_grid, to_map, Num};

type Position = Complex<Num>;

type Velocity = Complex<Num>;

type Particle = (Position, Velocity);

fn parse(s: &str) -> Vec<Particle> {
    s.lines().map(read_row).collect()
}

fn read_row(row: &str) -> Particle {
    let parts: Vec<Num> = row
        .replace([',', '<', '>'], " ")
        .split(' ')
        .filter_map(|n| n.trim().parse::<Num>().ok())
        .collect();
    (
        Position::new(parts[0], parts[1]),
        Velocity::new(parts[2], parts[3]),
    )
}

fn at_time(stars: &HashSet<Particle>, time: Num) -> HashSet<Position> {
    stars.iter().map(|(p, d)| p + (d.scale(time))).collect()
}

fn find_best(points: &[Particle]) -> Num {
    let (a, b) = points[0];
    let (c, d) = points[1];
    let (dist, vel) = (a - c, (b - d).scale(-1));

    let posses = [dist.re.checked_div(vel.re), dist.im.checked_div(vel.im)];

    let prob = posses.into_iter().flatten().next().unwrap();

    //get result near the answer - assume it's
    //within 20 seconds of actual answer

    (prob - 20..=prob + 20)
        .min_by_key(|t| {
            let k = HashSet::from_iter(points.to_owned());
            let res = at_time(&k, *t);
            to_grid(&to_map(&res)).len()
        })
        .unwrap()
}

pub fn part1(s: &str) {
    let points = parse(s);

    let best = find_best(&points);

    let k = HashSet::from_iter(points);
    let res = at_time(&k, best);

    s_display(&to_map(&res));
    println!();
}

pub fn part2(s: &str) -> Num {
    let points = parse(s);

    find_best(&points)
}
