use itertools::Itertools as _;
use std::{
    fmt::Debug,
    io::{self, BufRead},
    str::FromStr,
};

struct Row {
    nums: Vec<i16>,
}

impl Row {
    fn draw(&mut self, draw: i16) {
        for cell in self.nums.iter_mut() {
            if *cell == draw {
                *cell = -1;
            }
        }
    }

    fn complete(&self) -> bool {
        self.nums.iter().all(|c| *c < 0)
    }

    fn score(&self) -> u32 {
        self.nums
            .iter()
            .filter_map(|c| if *c < 0 { None } else { Some(*c as u32) })
            .sum()
    }
}

impl FromStr for Row {
    type Err = ();

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        let mut nums = vec![];
        while !s.is_empty() {
            let (pref, suf) = s.split_at(2);
            nums.push(pref.trim().parse::<i16>().unwrap());
            if suf.is_empty() {
                s = suf;
            } else {
                s = suf.split_at(1).1;
            }
        }
        Ok(Self { nums })
    }
}

struct Table {
    lines: Vec<Row>,
}

impl Table {
    fn new<S: AsRef<str> + Debug, I: Iterator<Item = S>>(mut lines: I) -> Self {
        let first = lines.next().unwrap();
        assert!(first.as_ref().is_empty(), "{}", first.as_ref());

        Self {
            lines: lines
                .map(|s| Row::from_str(s.as_ref()).unwrap())
                .collect_vec(),
        }
    }

    fn draw(&mut self, draw: i16) -> Option<u32> {
        for line in &mut self.lines {
            line.draw(draw);
        }
        if self.lines.iter().any(|line| line.complete())
            | ((0..5).any(|n| self.lines.iter().all(|l| l.nums[n] < 0)))
        {
            Some(self.lines.iter().map(Row::score).sum())
        } else {
            None
        }
    }
}


fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let mut lines = stdin.lines().map(Result::unwrap);

    let draws: Vec<i16> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut tables = lines.chunks(6).into_iter().map(Table::new).collect_vec();

    'outer: for draw in draws.into_iter() {
        for tbl in tables.iter_mut() {
            if let Some(res) = tbl.draw(draw) {
                println!("{}", res * (draw as u32));
                break 'outer;
            }
        }
    }
}
