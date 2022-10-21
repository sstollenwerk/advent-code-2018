use std::collections::HashMap;

use chrono::Duration;
use chrono::Timelike;
use chrono::{NaiveDate, NaiveDateTime};
use itertools::Itertools;

type Num = u32;

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq, PartialOrd)]
enum Guard {
    Arrival(Num),
    Sleep,
    Wake,
}

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq, PartialOrd)]
struct Event {
    time: NaiveDateTime,
    event: Guard,
}

impl Event {
    fn new(time: NaiveDateTime, event: Guard) -> Event {
        Event { time, event }
    }
}

type Shift = Vec<Event>;
type Shifts = HashMap<Event, Shift>;

fn parse(s: &str) -> Vec<Event> {
    s.lines().sorted().map(read_row).collect()
}

fn read_row(row: &str) -> Event {
    let parts: Vec<String> = row.split(']').map(String::from).collect();
    let time = NaiveDateTime::parse_from_str(&parts[0][1..], "%Y-%m-%d %H:%M").unwrap();

    let event = match parts[1].as_str() {
        " falls asleep" => Guard::Sleep,
        " wakes up" => Guard::Wake,
        s => Guard::Arrival(
            s.split(' ').map(String::from).collect::<Vec<_>>()[2][1..]
                .parse::<Num>()
                .unwrap(),
        ),
    };
    Event::new(time, event)
}

fn group_guards(mut events: Vec<Event>) -> Shifts {
    let mut guard = None;
    let mut res: Shifts = HashMap::new();

    let mut currents = Vec::new();

    events.push(read_row("[1518-11-01 00:00] Guard #00 begins shift"));
    // placeholder
    for e in events.into_iter() {
        if let Guard::Arrival(_) = e.event {
            if let Some(d) = guard {
                res.insert(d, currents);
            };
            currents = Vec::new();
            guard = Some(e);
        }
        currents.push(e);
    }
    res
}

fn wakeful(s: &Shift) -> (Num, NaiveDate, Vec<bool>) {
    let a = s[0];
    let e = a.event;
    let n = if let Guard::Arrival(n) = e {
        n
    } else {
        unreachable!()
    };

    let mut t = a.time;
    let mut vals = Vec::new();
    let mut asleep = false;
    let events: HashMap<NaiveDateTime, Guard> = s.iter().map(|r| (r.time, r.event)).collect();

    while t.hour() == 0 {
        t -= Duration::minutes(1);
    }

    while t.hour() != 1 {
        if let Some(e) = events.get(&t) {
            match e {
                Guard::Arrival(_) => (),
                Guard::Sleep => asleep = true,
                Guard::Wake => asleep = false,
            };
        }
        if t.hour() == 0 {
            vals.push(asleep);
        }
        t += Duration::minutes(1);
    }
    (n, t.date(), vals)
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    // from https://stackoverflow.com/a/64499219

    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

pub fn part1(s: &str) -> Num {
    let vals = parse(s);

    let groups = group_guards(vals);

    let mut sleeps: HashMap<Num, Vec<Vec<bool>>> = HashMap::new();

    for e in groups.values() {
        let (guard, _, times) = wakeful(e);

        sleeps.entry(guard).or_default().push(times);
    }

    let (id, times) = sleeps
        .iter()
        .max_by_key(|(_, xs)| xs.iter().flatten().map(|&b| b as Num).sum::<Num>())
        .unwrap();

    let best = most_common_sleep(times).0;

    id * (best as Num)
}

fn most_common_sleep(times: &[Vec<bool>]) -> (usize, Num) {
    transpose(times.to_vec())
        .into_iter()
        .map(|x| x.iter().map(|&b| b as Num).sum::<Num>())
        .enumerate()
        .max_by_key(|t| t.1)
        .unwrap()
}

pub fn part2(s: &str) -> Num {
    let vals = parse(s);

    let groups = group_guards(vals);

    let mut sleeps: HashMap<Num, Vec<Vec<bool>>> = HashMap::new();

    for e in groups.values() {
        let (guard, _, times) = wakeful(e);

        sleeps.entry(guard).or_default().push(times);
    }

    let (id, v) = sleeps
        .iter()
        .max_by_key(|(_, xs)| most_common_sleep(xs).1)
        .unwrap();

    let (index, _) = most_common_sleep(v);

    id * (index as Num)
}
