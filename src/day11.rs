use std::collections::HashMap;

use num_complex::Complex;
use itertools::Itertools;

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

fn square(p: &Position) -> Vec<Position> {
    (0..3)
        .cartesian_product(0..3)
        .map(|(x, y)| Complex::new(x, y) + p)
        .collect()
}

pub fn part1(s: &str) -> Position {
    let serial = parse(s);
    dbg!(&serial);

    let pow = |pos| power(&serial, &pos);

    let cells = (1..=300).cartesian_product(1..=300).map(|(x, y)| Complex::new(x, y));

    let powers: HashMap<Position, Num> = cells.map(|pos| (pos, pow(pos))).collect();

    let total = |p: &Position| square(p).into_iter().map(|c| powers[&c]).sum::<Num>();

    let res = *powers
        .keys()
        .filter(|p| square(*p).into_iter().all(|c| powers.contains_key(&c)))
        .max_by_key(|p| total(p))
        .unwrap();


    dbg!(total(&res));
    dbg!(total(&Complex::new(21, 61)));
    // somehow have same answer as sample input???
    res
}

pub fn part2(s: &str) -> Num {
    todo!();
}
