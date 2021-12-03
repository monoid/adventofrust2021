use std::collections::HashMap;
use std::io::{self, BufRead as _};

fn main() {
    let mut counts = HashMap::<_, usize>::new();
    let mut lines = 0;
    let mut len = 0;

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();

        lines += 1;

        len = line.len();
        for (i, c) in line.chars().rev().enumerate() {
            let ref_ = counts.entry(i).or_default(); // Initialize no matter what
            if c == '1' {
                *ref_ += 1;
            }
        }
    }

    let gamma_str: String = (0..len)
        .rev()
        .map(|idx| {
            if counts.get(&idx).unwrap() > &(lines / 2) {
                '1'
            } else {
                '0'
            }
        })
        .collect();
    let gamma = u32::from_str_radix(&gamma_str, 2).unwrap();
    let epsilon = gamma ^ ((1 << len) - 1);

    println!("{}", gamma * &epsilon);
}
