use std::io::{self, BufRead};

fn cost(d: i32) -> i32 {
    d * (d + 1) / 2
}

fn total_cost(to: i32, pos: &[i32]) -> i32 {
    pos.iter().map(|p| cost((p - to).abs())).sum()
}

fn main() {
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).unwrap();
    let pos: Vec<i32> = line.trim().split(',').map(|s| s.parse().unwrap()).collect();

    let res = (*pos.iter().min().unwrap()..=*pos.iter().max().unwrap())
        .map(|to| total_cost(to, &pos))
        .min()
        .unwrap();

    println!("{}", res);
}
