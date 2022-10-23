use trees::{Node, Tree};

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

fn value(n: &Node<Vec<Num>>) -> Num {
    let meta = n.data().iter();

    if n.has_no_child() {
        meta.sum()
    } else {
        meta.filter(|m| m > &&0)
            .filter_map(|m| n.iter().nth(m - 1))
            .map(value)
            .sum()
    }

    // technically .filter(|m| m > &&0) is uneccesary because
    // 0_usize - 1 becomes 2.pow(32)-1 is longer than iter in input
    // so nth results in None
    // but clearer this way
}

pub fn part1(s: &str) -> Num {
    let vals = parse(s);

    let meta = vals.bfs().iter.flat_map(|n| n.data.clone());

    meta.sum()
}

pub fn part2(s: &str) -> Num {
    let vals = parse(s);

    value(vals.root())
}
