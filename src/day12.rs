use std::collections::HashMap;
use std::collections::HashSet;

use itertools::Itertools;
use itertools::MinMaxResult::{MinMax, NoElements, OneElement};

type State = HashSet<i32>;
type Map = HashMap<Vec<bool>, bool>;

fn exists(c: char) -> bool {
    c == '#'
}

fn as_row(s: &str) -> Vec<bool> {
    s.chars().map(exists).collect()
}

fn parse(s: &str) -> (State, Map) {
    let mut parts = s.lines();

    let state: State = as_row(parts.next().unwrap().split(' ').nth(2).unwrap())
        .into_iter()
        .enumerate()
        .filter_map(|(i, el)| if el { Some(i as i32) } else { None })
        .collect();
    parts.next();

    (state, parts.map(read_row).collect())
}

fn read_row(row: &str) -> (Vec<bool>, bool) {
    let mut parts = row.split(" => ");
    let a = as_row(parts.next().unwrap());
    let b = exists(parts.next().unwrap().chars().next().unwrap());
    (a, b)
}

fn step(trans: &Map, state: &State) -> State {
    if state.is_empty() {
        return State::new();
    }

    let (start, end) = match state.iter().minmax() {
        NoElements => unreachable!(),
        OneElement(a) => (a, a),
        MinMax(a, b) => (a, b),
    };
    (start - 5..end + 5)
        .filter(|n| {
            let adj = (n - 2..=n + 2)
                .map(|k| state.contains(&k))
                .collect::<Vec<bool>>();
            *trans.get(&adj).unwrap_or(&false)
        })
        .collect()
}

pub fn part1(s: &str) -> i32 {
    let (mut state, trans) = parse(s);
    for _ in 0..20 {
        state = step(&trans, &state)
    }

    state.into_iter().sum()
}

pub fn part2(s: &str) -> i32 {
    todo!();
}
