use std::{
    collections::{HashSet, VecDeque},
    io::BufRead,
};

pub type Pt = (isize, isize);
pub type Cell = u8;

pub const START: Cell = 0;
pub const END: Cell = 9;

#[derive(Debug)]
pub struct Map {
    map: Vec<Vec<Cell>>,
}

impl Map {
    pub fn read<R: BufRead>(inp: R) -> Self {
        let map = inp
            .lines()
            .map(|r| {
                let line = r.unwrap();
                let line = line.trim();
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as Cell)
                    .collect()
            })
            .collect();
        Self { map }
    }

    pub fn solve1(&self) -> usize {
        let mut sum = 0;
        for (y, row) in self.map.iter().enumerate() {
            for (x, val) in row.iter().cloned().enumerate() {
                if val == START {
                    sum += self.find_ends1((x as isize, y as isize));
                }
            }
        }
        sum
    }

    pub fn solve2(&self) -> usize {
        let mut sum = 0;
        for (y, row) in self.map.iter().enumerate() {
            for (x, val) in row.iter().cloned().enumerate() {
                if val == START {
                    sum += self.find_trails2((x as isize, y as isize));
                }
            }
        }
        sum
    }

    fn find_ends1(&self, p: Pt) -> usize {
        let start_val = 0;
        let mut ends = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((start_val, p));

        while let Some((val, p)) = queue.pop_back() {
            if val == END {
                ends.insert(p);
            } else {
                for (nv, np) in self.go_up(p, val) {
                    queue.push_back((nv, np));
                }
            }
        }

        ends.len()
    }

    fn find_trails2(&self, p: Pt) -> usize {
        let start_val = 0;
        let mut ends = 0;
        let mut queue = VecDeque::new();
        queue.push_back((start_val, p));

        while let Some((val, p)) = queue.pop_back() {
            if val == END {
                ends += 1;
            } else {
                for (nv, np) in self.go_up(p, val) {
                    queue.push_back((nv, np));
                }
            }
        }

        ends
    }

    pub fn go_up(&self, p: Pt, val: Cell) -> impl Iterator<Item = (Cell, (isize, isize))> + '_ {
        let next_val = val + 1;
        self.around(p).filter_map(move |np| {
            if let Some(nv) = self.get(np) {
                if nv == next_val {
                    return Some((next_val, np));
                }
            }
            None
        })
    }

    pub fn get(&self, p: Pt) -> Option<Cell> {
        let upt: Option<(usize, usize)> = p.0.try_into().ok().zip(p.1.try_into().ok());
        upt.and_then(|(ux, uy)| self.map.get(uy).and_then(|row| row.get(ux).copied()))
    }

    pub fn around(&self, p: Pt) -> impl Iterator<Item = (isize, isize)> {
        [(0, 1), (1, 0), (0, -1), (-1, 0)]
            .into_iter()
            .map(move |(dx, dy)| {
                let nx = p.0 + dx;
                let ny = p.1 + dy;
                (nx, ny)
            })
    }
}
