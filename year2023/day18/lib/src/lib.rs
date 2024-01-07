use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Instr {
    pub dir: Dir,
    pub len: usize,
    pub col: String,
}

impl Instr {
    pub fn into_v2(self) -> Self {
        let dir = match self.col.as_bytes()[5] {
            b'0' => Dir::R,
            b'1' => Dir::D,
            b'2' => Dir::L,
            b'3' => Dir::U,
            _ => panic!("incorrect data {:?}", self.col),
        };

        let len = usize::from_str_radix(self.col.split_at(5).0, 16).unwrap();

        Self { dir, len, ..self }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Dir {
    #[default]
    L,
    R,
    U,
    D,
}

impl Dir {
    fn from_chr(c: char) -> Self {
        match c {
            'U' => Self::U,
            'D' => Self::D,
            'L' => Self::L,
            'R' => Self::R,
            _ => panic!("WTF"),
        }
    }

    fn step(self) -> (isize, isize) {
        match self {
            Dir::U => (0, -1),
            Dir::D => (0, 1),
            Dir::L => (-1, 0),
            Dir::R => (1, 0),
        }
    }
}

fn parse_instr(inp: &str) -> Instr {
    let (dirs, rest) = inp.split_once(' ').unwrap();
    let dir = Dir::from_chr(dirs.chars().next().unwrap());

    let (lens, cols) = rest.split_once(' ').unwrap();
    let len = lens.parse().unwrap();

    let col = cols
        .strip_prefix('(')
        .unwrap()
        .strip_suffix(')')
        .unwrap()
        .strip_prefix('#')
        .unwrap()
        .to_owned();

    Instr { dir, len, col }
}

pub fn read_data() -> Vec<Instr> {
    std::io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            parse_instr(line.trim())
        })
        .collect()
}

pub fn read_data2() -> Vec<Instr> {
    let data = read_data();
    data.into_iter().map(Instr::into_v2).collect()
}

pub type Pos = (isize, isize);

#[derive(Debug, Default, Clone)]
pub struct Turtle {
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,

    pos: Pos,

    track: HashMap<Pos, Dir>,
}

impl Turtle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn move_(&mut self, instr: &Instr) {
        let step = instr.dir.step();
        for _ in 0..instr.len {
            self.advance(step, instr.dir);
        }
    }

    pub fn advance(&mut self, delta: (isize, isize), dir: Dir) {
        use std::cmp::{max, min};

        let r = self.track.entry(self.pos).or_default();
        *r = max(dir, *r);

        self.pos.0 += delta.0;
        self.pos.1 += delta.1;

        let r = self.track.entry(self.pos).or_default();
        *r = max(dir, *r);

        self.min_x = min(self.min_x, self.pos.0);
        self.max_x = max(self.max_x, self.pos.0);
        self.min_y = min(self.min_y, self.pos.1);
        self.max_y = max(self.max_y, self.pos.1);
    }

    pub fn get_min(&self) -> Pos {
        (self.min_x, self.min_y)
    }

    pub fn get_max(&self) -> Pos {
        (self.max_x, self.max_y)
    }

    pub fn into_track(self) -> HashMap<Pos, Dir> {
        self.track
    }

    pub fn into_sorted_track(self) -> Vec<(isize, Vec<(isize, Dir)>)> {
        use itertools::Itertools;

        let mut points: Vec<_> = self.track.into_iter().collect();
        points.sort_unstable_by_key(|&((x, y), _)| (y, x));

        points
            .into_iter()
            .map(|((x, y), dir)| (y, (x, dir)))
            .group_by(|&(y, _)| y)
            .into_iter()
            .map(|(k, g)| (k, g.map(snd).collect()))
            .collect()
    }
}

fn snd<A, B>((_, b): (A, B)) -> B {
    b
}

pub fn count_line(line: &[(isize, Dir)]) -> isize {
    let mut cnt = 0;
    let mut it = line.iter().cloned().peekable();
    'outer: while let Some(start) = it.next() {
        let mut prev = start;
        while let Some(&m) = it.peek() {
            match m.1 {
                Dir::L | Dir::R => {
                    assert_eq!(prev.0 + 1, m.0);
                    prev = m;
                    it.next();
                }
                Dir::U | Dir::D => {
                    if m.1 != start.1 || prev.0 + 1 == m.0 {
                        prev = m;
                        it.next();
                    } else {
                        cnt += prev.0 - start.0 + 1;
                        continue 'outer;
                    }
                }
            }
        }
        // tail
        cnt += prev.0 - start.0 + 1;
    }
    cnt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        assert_eq!(2, count_line(&vec![(0, Dir::U), (1, Dir::D)]));
    }

    #[test]
    fn test_u() {
        assert_eq!(3, count_line(&vec![(0, Dir::U), (2, Dir::D)]));
    }

    #[test]
    fn test_bar() {
        assert_eq!(3, count_line(&vec![(0, Dir::U), (1, Dir::L), (2, Dir::D)]));
    }
}
