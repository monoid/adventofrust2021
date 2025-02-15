use std::{collections::HashSet, io::BufRead};

#[derive(Debug, Clone)]
pub struct Map {
    pub data: Vec<Vec<bool>>,
    pub init_pos: (usize, usize),
}

impl Map {
    pub fn new(data: Vec<Vec<bool>>, init_pos: (usize, usize)) -> Self {
        Self { data, init_pos }
    }

    pub fn read<R: BufRead>(input: R) -> Self {
        let mut init_pos = (0, 0);
        let data = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.unwrap()
                    .chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '.' => false,
                        '#' => true,
                        '^' => {
                            init_pos.0 = x;
                            init_pos.1 = y;
                            false
                        }
                        _ => panic!("Invalid character"),
                    })
                    .collect()
            })
            .collect();

        Self { data, init_pos }
    }

    pub fn is_within_map(&self, pos: (isize, isize)) -> bool {
        pos.0 >= 0
            && pos.0 < self.data[0].len() as isize
            && pos.1 >= 0
            && pos.1 < self.data.len() as isize
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    pub fn move_me(self, pos: (isize, isize)) -> (isize, isize) {
        match self {
            Dir::Up => (pos.0, pos.1 - 1),
            Dir::Down => (pos.0, pos.1 + 1),
            Dir::Left => (pos.0 - 1, pos.1),
            Dir::Right => (pos.0 + 1, pos.1),
        }
    }

    pub fn turn_right(self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Right => Dir::Down,
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Guard {
    pub pos: (isize, isize),
    pub dir: Dir,
}

impl Guard {
    pub fn new(pos: (usize, usize), dir: Dir) -> Self {
        Self {
            pos: (pos.0 as _, pos.1 as _),
            dir,
        }
    }

    pub fn try_move(&mut self, map: &Map) -> Option<(isize, isize)> {
        let new_pos = self.dir.move_me(self.pos);
        if !map.is_within_map(new_pos) {
            self.pos = new_pos;
            None
        } else {
            if !map.data[new_pos.1 as usize][new_pos.0 as usize] {
                self.pos = new_pos;
            } else {
                self.dir = self.dir.turn_right();
            }
            Some(self.pos)
        }
    }
}

pub fn find_visited(map: &Map) -> HashSet<(isize, isize)> {
    let mut guard = Guard::new(map.init_pos, Dir::Up);
    let mut pos = HashSet::new();

    pos.insert(guard.pos);
    while map.is_within_map(guard.pos) {
        if let Some(p) = guard.try_move(map) {
            pos.insert(p);
        }
    }
    pos
}
