use std::io;

// We have small values, it's ok
fn inv(x: u64, a: u64) -> u64 {
    for i in 1..a {
        if x * i % a == 1 {
            return i;
        }
    }
    panic!("Failed to invert");
}

fn crt(constraints: &[(u64, u64)]) -> u64 {
    let m: u64 = constraints.iter().map(|(_, ai)| ai).product();
    constraints
        .iter()
        .map(|&(ri, ai)| {
            let mi = m / ai;
            ri * mi * inv(mi, ai)
        })
        .sum::<u64>()
        % m
}

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let (_, data) = lib::parse_input(&mut stdin);

    let constraints: Vec<_> = data
        .into_iter()
        .enumerate()
        .filter_map(|(i, v)| v.map(move |v| ((v - (i % v)) as u64, v as u64)))
        .collect();

    println!("{}", crt(&constraints));
}
