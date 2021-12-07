use std::io::{self, BufRead};

fn cost(to: i32, pos: &[i32]) -> i32 {
    pos.iter().map(|p| (p - to).abs()).sum()
}

fn main() {
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).unwrap();
    let pos: Vec<i32> = line.trim().split(',').map(|s| s.parse().unwrap()).collect();

    let res = (*pos.iter().min().unwrap()..=*pos.iter().max().unwrap())
        .map(|to| cost(to, &pos))
        .min()
        .unwrap();

    println!("{}", res);
}
