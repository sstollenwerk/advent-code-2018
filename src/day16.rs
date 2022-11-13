use std::collections::HashMap;
use std::collections::HashSet;

use crate::day16::Opcode::{Add, And, Equ, Gre, Mul, Or, Set};
use crate::day16::V::{C, N};

type Num = i32;
type Register = usize;

type Registers = Vec<Num>;

#[derive(Debug, Eq, PartialEq, Clone, Hash, Ord, PartialOrd, Copy)]
enum V {
    N(Num),
    C(Register),
}

#[derive(Debug, Eq, PartialEq, Clone, Hash, Ord, PartialOrd, Copy)]
enum Opcode {
    Add,
    Mul,
    And,
    Or,
    Set,
    Gre,
    Equ,
}
type BaseInstruction = Vec<Num>;
type Instruction = (Opcode, V, V, V);

type Sample = (BaseInstruction, Registers, Registers);

fn parse(s: &str) -> (Vec<Sample>, Vec<BaseInstruction>) {
    let parts: Vec<&str> = s.split("\n\n\n\n").collect();
    let samples = parts[0].split("\n\n").map(read_sample).collect();
    let inst = parts[1].lines().map(read_instruction).collect();

    (samples, inst)
}

fn read_sample(row: &str) -> Sample {
    let parts: Vec<_> = row.lines().collect();
    (
        read_instruction(parts[1]),
        read_reg(parts[0]),
        read_reg(parts[2]),
    )
}

fn read_instruction(row: &str) -> BaseInstruction {
    row.trim()
        .split(' ')
        .map(|n| n.trim().parse::<Num>().unwrap())
        .collect()
}

fn read_reg(row: &str) -> Registers {
    let a = row.split_once(' ').unwrap().1.replace(['[', ']', ','], "");
    read_instruction(&a)
}

fn as_num(r: &Registers, k: V) -> Num {
    match k {
        V::N(n) => n as Num,
        V::C(c) => r[c],
    }
}

fn interpret(r: &Registers, c: &Instruction) -> Registers {
    let mut reg = r.clone();

    let a = as_num(r, c.1);
    let b = as_num(r, c.2);

    let res = match c.0 {
        Add => a + b,
        Mul => a * b,
        And => a & b,
        Or => a | b,
        Set => a,
        Gre => (a > b) as Num,
        Equ => (a == b) as Num,
    };

    if let C(p) = c.3 {
        reg[p] = res;
    } else {
        unreachable!();
    }

    reg
}

fn run(inst: Vec<Instruction>) -> Registers {
    let mut r: Registers = vec![0; 4];

    for i in inst {
        r = interpret(&r, &i);
    }
    r
}

fn op_posses(r_: &BaseInstruction) -> Vec<Instruction> {
    let r = r_.clone();
    let mut res: Vec<Instruction> = Vec::new();
    let ops = vec![Add, Mul, And, Or, Set, Gre, Equ];
    for (i, op) in ops.into_iter().enumerate() {
        res.push((op, C(r[1] as usize), C(r[2] as usize), C(r[3] as usize)));
        if i != 4 {
            res.push((op, C(r[1] as usize), N(r[2]), C(r[3] as usize)));
        }
        if i >= 4 {
            res.push((op, N(r[1]), C(r[2] as usize), C(r[3] as usize)));
        }
    }
    assert!(res.len() == 16);

    res
}

fn act_posses(s: &Sample) -> HashSet<usize> {
    let mut res = HashSet::new();
    let (inst, start, end) = s;
    for (i, poss) in op_posses(inst).into_iter().enumerate() {
        if interpret(start, &poss) == *end {
            res.insert(i);
        }
    }
    assert!(!res.is_empty());
    res
}

pub fn part1(s: &str) -> usize {
    let (samples, _) = parse(s);

    samples.iter().filter(|x| act_posses(x).len() >= 3).count()
}

fn find_possibilities(samples: Vec<Sample>) -> HashMap<Num, usize> {
    let mut posses: HashMap<Num, HashSet<usize>> = HashMap::new();

    for s in samples {
        let k = s.0[0];
        let v = act_posses(&s);
        if !posses.contains_key(&k) {
            posses.insert(k, v);
        } else {
            posses.insert(k, &posses[&k] & &v);
        }
    }

    let mut res: HashMap<Num, usize> = HashMap::new();

    while !posses.is_empty() {
        let tmp = posses.clone();
        let (k, v) = tmp.iter().find(|(_, v)| v.len() == 1).unwrap();
        posses.remove(k);
        res.insert(*k, *v.iter().next().unwrap());

        let tmp = posses.clone();

        for k2 in tmp.keys() {
            posses.insert(*k2, &posses[k2] - v);
        }
    }

    res
}

pub fn part2(s: &str) -> Num {
    let (samples, inst) = parse(s);

    let map = find_possibilities(samples);

    let instructions: Vec<Instruction> = inst
        .into_iter()
        .map(|x| op_posses(&x)[map[&x[0]]])
        .collect();

    run(instructions)[0]
}
