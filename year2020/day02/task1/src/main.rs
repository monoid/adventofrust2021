use std::io::{self, BufRead as _};

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    eprintln!(
        "{}",
        stdin
            .lines()
            .filter(|line| {
                let line = line.as_ref().unwrap();
                let (pol, pass) = lib::parse_line(line).unwrap();
                pol.verify1(pass)
            })
            .count()
    );
}
