use std::{
    collections::HashSet,
    io::{self, BufRead as _},
};

#[derive(Copy, Clone)]
enum Instruction {
    X(isize),
    Y(isize),
}

fn fold<'a, T: IntoIterator<Item = &'a (isize, isize)> + 'a>(
    pairs: T,
    ins: Instruction,
) -> HashSet<(isize, isize)> {
    let mut res = HashSet::new();
    for &(x, y) in pairs.into_iter() {
        match ins {
            Instruction::X(xa) => {
                if x <= xa {
                    res.insert((x, y));
                } else {
                    res.insert((xa - (x - xa), y));
                }
            }
            Instruction::Y(ya) => {
                if y <= ya {
                    res.insert((x, y));
                } else {
                    res.insert((x, ya - (y - ya)));
                }
            }
        }
    }
    res
}

fn main() {
    let mut points = Vec::new();
    let mut folds = Vec::new();

    let stdin = io::stdin();
    let stdin = stdin.lock();
    let mut lines = stdin.lines();
    loop {
        let line = lines.next().unwrap().unwrap();
        if line.is_empty() {
            break;
        }
        let (x, y) = line.split_once(',').unwrap();
        points.push((x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap()));
    }

    while let Some(line) = lines.next() {
        let line = line.unwrap();
        if line.starts_with("fold along ") {
            let (p, a) = line.split_once('=').unwrap();
            if p.ends_with('x') {
                folds.push(Instruction::X(a.parse().unwrap()));
            } else if p.ends_with('y') {
                folds.push(Instruction::Y(a.parse().unwrap()));
            } else {
                panic!("{}", line);
            }
        }
    }

    let folded = fold(&points, folds[0]);

    println!("{}", folded.len())
}
