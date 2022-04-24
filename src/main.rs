#![feature(is_sorted)]
#![allow(unused_parens)]

mod day07;
use day07::{part1, part2};
mod lib;

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}
