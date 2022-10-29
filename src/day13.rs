use std::collections::HashMap;
use std::collections::HashSet;

use num_complex::Complex;

use crate::helper::{read_grid, Num};

type Position = Complex<Num>;

type Direction = Complex<Num>;

type Carts = HashMap<Position, (Direction, usize)>;
type Track = HashMap<Position, HashSet<Position>>;

//const DOWN: Direction = Complex::new(0, 1);

const LEFT: Direction = Complex::new(0, -1);
const STRAIGHT: Direction = Complex::new(1, 0);
const RIGHT: Direction = Complex::new(0, 1);

fn parse(s: &str) -> (Track, Carts) {
    let positions = read_grid(s);

    let directions = HashMap::from([
        ('v', Complex::new(0, 1)),
        ('^', Complex::new(0, -1)),
        ('>', Complex::new(1, 0)),
        ('<', Complex::new(-1, -0)),
    ]);

    let mut track: Track = Track::new();
    let horiz = positions
        .iter()
        .filter(|(_, &v)| "-><".contains(v))
        .map(|(&k, _)| {
            (
                k,
                HashSet::from([k + Complex::new(1, 0), k + Complex::new(-1, 0)]),
            )
        });
    track.extend(horiz);

    let vert = positions
        .iter()
        .filter(|(_, &v)| "|v^".contains(v))
        .map(|(&k, _)| {
            (
                k,
                HashSet::from([k + Complex::new(0, 1), k + Complex::new(0, -1)]),
            )
        });
    track.extend(vert);

    let inter = positions
        .iter()
        .filter(|(_, &v)| v == '+')
        .map(|(&k, _)| (k, adj(&k)));
    track.extend(inter);

    let t = track.clone();

    let turns = positions
        .iter()
        .filter(|(_, &v)| v == '\\' || v == '/')
        .map(|(&k, _)| {
            (
                k,
                adj(&k)
                    .into_iter()
                    .filter(|p| t.get(p).unwrap_or(&HashSet::new()).contains(&k))
                    .collect(),
            )
        });
    track.extend(turns);

    let carts: Carts = positions
        .iter()
        .filter(|(_, v)| directions.contains_key(v))
        .map(|(k, v)| (*k, (directions[v], 0)))
        .collect();

    (track, carts)
}

fn adj(p: &Position) -> HashSet<Position> {
    [
        Complex::new(0, 1),
        Complex::new(0, -1),
        Complex::new(1, 0),
        Complex::new(-1, 0),
    ]
    .into_iter()
    .map(|c| p + c)
    .collect()
}

fn step(t: &Track, carts_: Carts) -> (Carts, HashSet<Position>) {
    let mut res: Carts = Carts::new();
    let mut crashed: HashSet<Position> = HashSet::new();
    // guessing I'll be wanting to run simulation even after first collision

    let mut currents: HashSet<Position> = carts_.keys().copied().collect();

    let mut carts: Vec<_> = carts_.into_iter().collect();
    carts.sort_by_key(|(pos, _)| (pos.im, pos.re));

    let moves = [LEFT, STRAIGHT, RIGHT];

    for (mut cart, (mut direction, mut num_intersections)) in carts.into_iter() {
        currents.remove(&cart);
        if crashed.contains(&cart) {
            continue;
        }

        let mut dirs = t[&cart].clone();
        dirs.remove(&(cart - direction));

        if dirs.len() == 1 {
            let place = dirs.drain().next().unwrap();
            direction = place - cart;
        } else {
            assert!(dirs.len() == 3);
            direction *= moves[num_intersections];
            num_intersections += 1;
            num_intersections %= moves.len();
        }

        cart += direction;

        if currents.contains(&cart) {
            crashed.insert(cart);
            currents.remove(&cart);
        } else if res.insert(cart, (direction, num_intersections)).is_some() {
            res.remove(&cart);
            crashed.insert(cart);
            currents.remove(&cart);
        } else {
            currents.insert(cart);
        }
    }
    for c in crashed.iter() {
        res.remove(c);
    }
    (res, crashed)
}

pub fn part1(s: &str) -> HashSet<Position> {
    let (track, mut carts) = parse(s);

    loop {
        let (c_, crashed) = step(&track, carts);
        carts = c_;
        /* println!("{:?}", &carts);

        let tx: HashSet<Position> = track.keys().copied().collect();
        let tx = tx
            .difference(&carts.keys().copied().collect())
            .copied()
            .collect();

        s_display(&to_map(&tx)); */

        if !crashed.is_empty() {
            return crashed;
        }
    }
}

pub fn part2(s: &str) -> Position {
    let (track, mut carts) = parse(s);

    loop {
        let (c_, _) = step(&track, carts);
        carts = c_;

        if carts.len() == 1 {
            return *carts.keys().next().unwrap();
        }
    }
}
