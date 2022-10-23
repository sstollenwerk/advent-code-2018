#![allow(dead_code)]

pub type Num = i64;

use num_complex::Complex;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn to_grid<V>(data: &HashMap<Complex<Num>, V>) -> Vec<Vec<&V>> {
    let largest = data.keys().map(|c| (c.re, c.im)).max().unwrap();

    let mut grid = Vec::new();

    for y in 0..=largest.1 {
        let mut res = Vec::new();

        for x in 0..=largest.0 {
            let d = &data[&Complex::new(x as Num, y as Num)];
            res.push(d);
        }
        grid.push(res);
    }

    grid
}

pub fn upside_down(data: &HashSet<Complex<Num>>) -> HashSet<Complex<Num>> {
    let mut res = HashSet::new();
    let ys: HashSet<_> = data.iter().map(|c| c.im).collect();
    if data.is_empty() {
        return res;
    }
    let ma = ys.iter().max().unwrap();
    let mi = ys.iter().min().unwrap();

    for c in data.iter() {
        let y = (ma - c.im) + mi;
        res.insert(Complex::new(c.re, y));
    }

    res
}

pub fn to_map(data: &HashSet<Complex<Num>>) -> HashMap<Complex<Num>, char> {
    let mut res = HashMap::new();

    if data.is_empty() {
        return res;
    }

    let ys: HashSet<_> = data.iter().map(|c| c.im).collect();
    let xs: HashSet<_> = data.iter().map(|c| c.re).collect();

    let top_left = Complex::new(*xs.iter().min().unwrap(), *ys.iter().min().unwrap());
    let bottom_right = Complex::new(*xs.iter().max().unwrap(), *ys.iter().max().unwrap());

    let size = bottom_right - top_left;

    for x in 0..=size.re {
        for y in 0..=size.im {
            let p = Complex::new(x, y);
            let contained = data.contains(&(p + top_left));

            let c = match contained {
                false => ' ',
                true => '#',
            };

            res.insert(p, c);
        }
    }

    res
}

pub fn s_display(data: &HashMap<Complex<Num>, char>) {
    for i in to_grid(data) {
        println!("{:}", i.iter().cloned().collect::<String>());
    }
    println!();
}

pub fn display<V: std::fmt::Debug>(data: &HashMap<Complex<Num>, V>) {
    for i in to_grid(data) {
        //  println!("{:}", i.iter().cloned().collect::<String>());
        println!("{:?}", i);
    }
    println!();
}
