type Num = usize;

fn parse(s: &str) -> Num {
    s.parse::<Num>().unwrap()
}

fn digits(n: Num) -> Vec<Num> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as Num)
        .collect()
}

fn recipes(amt: Num) -> Num {
    let mut a: usize = 0;
    let mut b: usize = 1;

    let mut vals = vec![3, 7];

    while vals.len() < amt + 10 {
        vals.extend(digits(vals[a] + vals[b]));

        a += vals[a] + 1;
        b += vals[b] + 1;
        a %= vals.len();
        b %= vals.len();
    }
    let res = &vals[amt..amt + 10];
    let show = res
        .iter()
        .map(|&n| char::from_digit(n as u32, 10).unwrap())
        .collect::<String>();
    parse(&show)
}

pub fn part1(s: &str) -> Num {
    let amt = parse(s);

    recipes(amt)
}

pub fn part2(s: &str) -> Num {
    todo!();
}
