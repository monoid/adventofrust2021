use std::{
    collections::HashMap,
    io::{self, BufRead as _},
    str::FromStr,
};

#[derive(Debug)]
struct Line {
    p0: (i32, i32),
    p1: (i32, i32),
}

impl FromStr for Line {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn split_pair(p: &str) -> (i32, i32) {
            let mut by_comma = p.split(',');
            let x = by_comma.next().unwrap().parse().unwrap();
            let y = by_comma.next().unwrap().parse().unwrap();
            (x, y)
        }
        let mut by_space = s.split(' ');
        let pair0 = by_space.next().unwrap();
        by_space.next().unwrap(); // Arrow
        let pair1 = by_space.next().unwrap();

        Ok(Self {
            p0: split_pair(pair0),
            p1: split_pair(pair1),
        })
    }
}

impl Line {
    fn is_simple(&self) -> bool {
        (self.p0.0 == self.p1.0) | (self.p0.1 == self.p1.1)
    }

    fn get_points(&self) -> impl Iterator<Item = (i32, i32)> + 'static {
        let p0: (_, _) = self.p0;
        let p1 = self.p1;

        let ax = (p1.0 - p0.0).abs();
        let ay = (p1.1 - p0.1).abs();

        let dx = (p1.0 - p0.0).signum();
        let dy = (p1.1 - p0.1).signum();

        let len = std::cmp::max(ax, ay);

        (0..=len).map(move |i| (p0.0 + i * dx, p0.1 + i * dy))
    }
}

fn main() {
    let mut points = HashMap::<_, u32>::new();
    for line_str in io::stdin().lock().lines() {
        let line = Line::from_str(&line_str.unwrap()).unwrap();
        if line.is_simple() {
            for p in line.get_points() {
                *points.entry(p).or_default() += 1;
            }
        }
    }

    let res = points
        .into_values()
        .filter_map(|v| if v > 1 { Some(()) } else { None })
        .count();
    println!("{}", res);
}
