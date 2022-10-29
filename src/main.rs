#![feature(is_sorted)]
#![allow(unused_parens)]

mod day13;
use day13::{part1, part2};
mod helper;
use std::fs;

fn read_day(day: i32) -> String {
    let filename = to_filename(day);
    fs::read_to_string(filename).expect("Could not read file")
}

fn to_filename(day: i32) -> String {
    format!("input/{:0>2}.txt", day)
}

fn main() {
    let data = read_day(13);
    println!("{:?}", &part1(&data));
    println!("{:?}", &part2(&data));
}
