use std::collections::HashMap;
use std::collections::HashSet;

use num_complex::Complex;

use crate::helper::{read_grid, s_display, to_map, Num};

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

        /*   if inter.contains(&cart) {
            direction *= moves[num_intersections];
            num_intersections += 1;
            num_intersections %= moves.len();
        } else if t.contains(&(cart + direction)) {
        } else {
            let mut around = &adj(&cart) & &t;
            around.remove(&(cart - direction));

            assert!(around.len() == 1);
            let place = around.into_iter().next().unwrap();
            direction = place - cart;
        }*/
        cart += direction;

        if currents.contains(&cart) {
            crashed.insert(cart);
        } else if let Some((c, _)) = res.insert(cart, (direction, num_intersections)) {
            res.remove(&cart);
            crashed.insert(cart);
        }

        currents.insert(cart);
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

pub fn part2(s: &str) -> Num {
    todo!();
}
