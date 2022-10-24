use std::collections::HashMap;
use std::collections::HashSet;

use itertools::Itertools;
use itertools::MinMaxResult::{MinMax, NoElements, OneElement};

type Num = i64;

type State = HashSet<Num>;
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
        .filter_map(|(i, el)| if el { Some(i as Num) } else { None })
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

pub fn part1(s: &str) -> Num {
    let (mut state, trans) = parse(s);
    for _ in 0..20 {
        state = step(&trans, &state);
    }

    state.into_iter().sum()
}

fn as_adj(state: &State) -> Vec<Num> {
    let k = state.iter().min().unwrap();
    state.iter().map(|c| c - k).sorted().collect()
}

fn score(adj: &Vec<Num>, t: Num) -> Num {
    assert!(adj[0] == 0);
    adj.iter().sum::<Num>() + t * (adj.len() as Num)
}

pub fn part2(s: &str) -> Num {
    let (mut state, trans) = parse(s);

    let mut seen: HashMap<Vec<Num>, (Num, usize)> = HashMap::new();
    let steps = 50000000000;
    for i in 0..steps {
        state = step(&trans, &state);

        let k = state.iter().min().unwrap();
        let adj: Vec<Num> = as_adj(&state);

        assert!(state.iter().sum::<Num>() == score(&adj, *k));

        let current = (*k, i);

        if let Some(prev) = seen.insert(adj.clone(), current) {
            dbg!(&current);
            dbg!(&prev);

            let delta = current.0 - prev.0;
            let time = current.1 - prev.1;

            let remaining = steps - (i + 1);

            assert!(remaining % time == 0);

            let shift = k + (delta * (remaining / time) as Num);
            return score(&adj, shift);
        }
    }

    unreachable!();

    //state.into_iter().sum()
}
