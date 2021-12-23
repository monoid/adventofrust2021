pub mod cube;
use cube::Cuboid;
use std::{
    io::{self, BufRead as _},
    ops::RangeInclusive,
    str::FromStr,
};

pub struct Command {
    pub state: bool,
    pub cuboid: Cuboid,
}

fn parse_range<T: FromStr>(range: &str) -> RangeInclusive<T>
where
    <T as FromStr>::Err: std::fmt::Debug,
{
    let range = range.split_once('=').unwrap().1;
    let mut items = range.split('.');
    let from_ = items.next().unwrap().parse().unwrap();
    items.next().unwrap();
    let to_ = items.next().unwrap().parse().unwrap();
    from_..=to_
}

pub fn prase_input() -> Vec<Command> {
    io::stdin()
        .lock()
        .lines()
        .map(|s| parse_line(&s.unwrap()))
        .collect()
}

pub fn parse_line(line: &str) -> Command {
    let mut line = line.trim();
    let state;
    if line.starts_with("on ") {
        state = true;
    } else {
        state = false;
    }
    line = line.split_once(' ').unwrap().1;
    let mut items = line.split(',');
    let x = items.next().unwrap();
    let y = items.next().unwrap();
    let z = items.next().unwrap();

    Command {
        state,
        cuboid: Cuboid {
            x: parse_range(x),
            y: parse_range(y),
            z: parse_range(z),
        },
    }
}
