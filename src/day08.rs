use trees::Tree;

type Num = usize;

type Tr = trees::Tree<Vec<Num>>;

fn parse(s: &str) -> Tr {
    let vals: Vec<Num> = s
        .trim()
        .split(' ')
        .map(|n| n.trim().parse::<Num>().unwrap())
        .collect();

    let (tree, remaining) = parse_section(&vals);
    assert!(remaining.is_empty());
    tree
}

fn parse_section(xs_: &[Num]) -> (Tr, Vec<Num>) {
    let mut xs = xs_.to_owned();

    assert!(xs.len() >= 2);

    let childs = xs[0];
    let metas = xs[1];

    xs = xs[2..].to_vec();

    let mut seen: Vec<Tr> = Vec::new();

    for _ in (0..childs) {
        let (t, rest) = parse_section(&xs);

        xs = rest;
        seen.push(t);
    }
    let data = xs[..metas].to_owned();
    xs = xs[metas..].to_vec();

    let mut tree = Tree::new(data);
    for t in seen.into_iter() {
        tree.push_back(t);
    }

    (tree, xs)
}

pub fn part1(s: &str) -> Num {
    let vals = parse(s);

    let meta = vals.bfs().iter.flat_map(|n| n.data.clone());

    meta.sum()
}

pub fn part2(s: &str) -> u32 {
    todo!();
}
