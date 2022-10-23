use std::collections::VecDeque;

type Num = usize;

fn parse(s: &str) -> (Num, Num) {
    let mut vals = s.split(' ').filter_map(|n| n.parse::<Num>().ok());

    (vals.next().unwrap(), vals.next().unwrap())
}

fn brute_force_game(players: Num, turns: Num) -> Num {
    let mut scores = vec![0; players + 1];

    let mut marbles: VecDeque<Num> = VecDeque::from([0]);

    let play = (1..=players).cycle();

    for (m, p) in (1..=turns).zip(play) {
        if m % 23 != 0 {
            marbles.rotate_left(2 % marbles.len());
            marbles.push_front(m);
        } else {
            //println!("{:?}",&marbles);
            scores[p] += m;
            marbles.rotate_right(7 % marbles.len());
            let k = marbles.pop_front().unwrap();
            scores[p] += k;
        }
        //println!("{:?}",&(p,m,i,marbles[i]));
        //println!("{:?}",&marbles);
        //println!();
    }
    println!("{:?}", &scores);
    *scores.iter().max().unwrap()
}

pub fn part1(s: &str) -> Num {
    let (players, turns) = parse(s);
    dbg!(players, turns);
    brute_force_game(players, turns)
}

pub fn part2(s: &str) -> Num {
    todo!();
}
