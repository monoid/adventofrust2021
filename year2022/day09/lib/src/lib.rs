use std::{collections::HashSet, str::FromStr};

#[derive(Default, Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub struct Point(isize, isize);

impl Point {
    fn is_touching(head: &Point, tail: &Point) -> bool {
        ((head.0 - tail.0).abs() <= 1) && ((head.1 - tail.1).abs() <= 1)
    }
}

#[derive(Debug)]
pub struct Rope {
    nodes: Vec<Point>,
}

impl Rope {
    pub fn new(len: usize) -> Self {
        Self {
            nodes: vec![<_>::default(); len],
        }
    }

    fn single_step(&mut self, dir: Dir) -> Point {
        self.move_head(dir);
        for mid in 1..self.nodes.len() {
            let (p, l) = self.nodes.split_at_mut(mid);
            Self::update_pair(p.last().unwrap(), l.first_mut().unwrap());
        }
        self.nodes.last().cloned().unwrap()
    }

    fn move_head(&mut self, dir: Dir) {
        let head = &mut self.nodes[0];
        match dir {
            Dir::R => head.0 += 1,
            Dir::L => head.0 -= 1,
            Dir::U => head.1 += 1,
            Dir::D => head.1 -= 1,
        }
    }

    fn update_pair(prev: &Point, next: &mut Point) {
        if Point::is_touching(prev, next) {
            // Nothing to do
            return;
        }
        // else
        if prev.0 == next.0 {
            if prev.1 < next.1 {
                next.1 = prev.1 + 1;
            } else {
                next.1 = prev.1 - 1;
            }
        } else if prev.1 == next.1 {
            if prev.0 < next.0 {
                next.0 = prev.0 + 1;
            } else {
                next.0 = prev.0 - 1;
            }
        } else {
            // diagonal
            let dx = (prev.0 - next.0).signum();
            let dy = (prev.1 - next.1).signum();
            assert!((dx != 0) && (dy != 0));
            next.0 += dx;
            next.1 += dy;
        }
    }

    pub fn execute_command(&mut self, command: (Dir, u32), trace: &mut HashSet<Point>) {
        let (dir, repeat) = command;
        for _ in 0..repeat {
            let tail_pos = self.single_step(dir);
            trace.insert(tail_pos);
        }
    }
}

#[derive(strum::EnumString, Debug, PartialEq, Eq, Copy, Clone)]
pub enum Dir {
    R,
    L,
    U,
    D,
}

pub fn read_data() -> Vec<(Dir, u32)> {
    use std::io::{self, BufRead};
    let stdin = io::stdin().lock();

    stdin
        .lines()
        .map(|r| parse_command(&r.unwrap()).unwrap().1)
        .collect()
}

fn parse_command(inp: &str) -> nom::IResult<&str, (Dir, u32)> {
    use nom::bytes::complete::tag;
    use nom::character::complete::u32;
    use nom::combinator::all_consuming;
    use nom::sequence::separated_pair;

    all_consuming(separated_pair(parse_dir, tag(" "), u32))(inp)
}

fn parse_dir(inp: &str) -> nom::IResult<&str, Dir> {
    use nom::combinator::map_res;

    map_res(nom::character::complete::anychar, |c| {
        Dir::from_str(&String::from(c))
    })(inp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command() {
        assert_eq!(parse_command("L 8"), Ok(("", (Dir::L, 8))));
        assert_eq!(parse_command("R 10"), Ok(("", (Dir::R, 10))));
        assert_eq!(parse_command("U 100"), Ok(("", (Dir::U, 100))));
        assert_eq!(parse_command("D 8"), Ok(("", (Dir::D, 8))));
    }
}
