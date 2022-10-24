use std::collections::HashMap;

use itertools::Itertools;
use num_complex::Complex;

use crate::helper::Num;

type Position = Complex<Num>;

fn parse(s: &str) -> Num {
    s.trim().parse::<Num>().unwrap()
}

fn power(serial: &Num, p: &Position) -> Num {
    let rack_id = p.re + 10;
    let mut power = rack_id * p.im;
    power += serial;
    power *= rack_id;

    power = (power % 1000) / 100;
    power - 5
}

fn square(p: &Position, n: Num) -> Vec<Position> {
    (0..n)
        .cartesian_product(0..n)
        .map(|(x, y)| Complex::new(x, y) + p)
        .collect()
}

pub fn part1(s: &str) -> Position {
    let serial = parse(s);
    dbg!(&serial);

    let pow = |pos| power(&serial, &pos);

    let cells = (1..=300)
        .cartesian_product(1..=300)
        .map(|(x, y)| Complex::new(x, y));

    let powers: HashMap<Position, Num> = cells.map(|pos| (pos, pow(pos))).collect();

    let total = |p: &Position| square(p, 3).into_iter().map(|c| powers[&c]).sum::<Num>();

    let res = *powers
        .keys()
        .filter(|p| powers.contains_key(&(*p + Complex::new(2, 2))))
        .max_by_key(|p| total(p))
        .unwrap();

    dbg!(total(&res));
    dbg!(total(&Complex::new(21, 61)));
    // somehow have same answer as sample input???
    res
}

fn edges(n: Num) -> Vec<Position> {
    let b = n - 1;
    let kx: Vec<Num> = (1..b).collect();

    kx.iter()
        .map(|a| Position::new(0, *a))
        .chain(kx.iter().map(|a| Position::new(b, *a)))
        .chain(kx.iter().map(|a| Position::new(*a, 0)))
        .chain(kx.iter().map(|a| Position::new(*a, b)))
        .collect()
}

pub fn part2(s: &str) -> (Position, Num) {
    dbg!(edges(3));
    let serial = parse(s);

    let pow = |pos| power(&serial, &pos);

    let cells = (1..=300)
        .cartesian_product(1..=300)
        .map(|(x, y)| Complex::new(x, y));

    let powers: HashMap<Position, Num> = cells.map(|pos| (pos, pow(pos))).collect();

    let mut totals: HashMap<(Position, Num), Num> = HashMap::new();
    let mut total = |p: &Position, n: Num| {
        let res = if n <= 2 {
            square(p, n).into_iter().map(|c| powers[&c]).sum::<Num>()
        } else {
            (totals[&(p + Complex::new(0, 0), n - 1)]
                + totals[&(p + Complex::new(0, 1), n - 1)]
                + totals[&(p + Complex::new(1, 0), n - 1)]
                + totals[&(p + Complex::new(1, 1), n - 1)]
                - 3 * totals[&(p + Complex::new(1, 1), n - 2)])
                - edges(n).iter().map(|e| powers[&(p + e)]).sum::<Num>()
        };
        totals.insert((*p, n), res);

        res
    };

    let r = (1..=300)
        .cartesian_product(powers.keys())
        .map(|(a, b)| (b, a))
        .filter(|(p, n)| powers.contains_key(&(*p + Complex::new(n - 1, n - 1))))
        .max_by_key(|(p, n)| total(p, *n))
        .unwrap();

    let res = (*r.0, r.1);

    dbg!(total(&res.0, res.1));
    dbg!(total(&Complex::new(232, 251), 12));

    res
}
