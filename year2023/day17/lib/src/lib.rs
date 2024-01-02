use std::cmp::Reverse;

use dary_heap::QuaternaryHeap as Heap;

use itertools::Either;

#[derive(Debug)]
pub struct Map {
    pub cells: Vec<Vec<u8>>,
    pub size: Pos,
}

impl Map {
    pub fn read_data() -> Self {
        let cells: Vec<Vec<u8>> = std::io::stdin()
            .lines()
            .map(|line| {
                let line = line.unwrap();
                line.trim().as_bytes().iter().map(|&c| c - b'0').collect()
            })
            .collect();

        let h = cells.len() as u16;
        let w = cells[0].len() as u16;

        Self {
            cells,
            size: (w, h),
        }
    }

    fn positions(
        &self,
        pos: Pos,
        dir: Dir,
        min: u16,
        max: u16,
    ) -> impl Iterator<Item = (Pos, u32)> + '_ {
        let items = (min..=max).flat_map(|n| [-(n as i16), n as i16].into_iter());
        let px = pos.0;
        let py = pos.1;
        let w = self.size.0;
        let h = self.size.1;

        match dir {
            Dir::V => Either::Left(
                items
                    .filter_map(move |dy| add(py, dy).filter(|&y| y < h))
                    .map(move |y| {
                        let dst = (px, y);
                        let score = self.dscore_v(pos, y);
                        (dst, score)
                    }),
            ),
            Dir::H => Either::Right(
                items
                    .filter_map(move |dx| add(px, dx).filter(|&x| x < w))
                    .map(move |x| {
                        let dst = (x, py);
                        let score = self.dscore_h(pos, x);
                        (dst, score)
                    }),
            ),
        }
    }

    fn dscore_v(&self, src: Pos, dest_y: u16) -> u32 {
        if dest_y < src.1 {
            (dest_y..src.1)
                .map(|y| self.cells[y as usize][src.0 as usize] as u32)
                .sum()
        } else {
            (src.1 + 1..=dest_y)
                .map(|y| self.cells[y as usize][src.0 as usize] as u32)
                .sum()
        }
    }

    fn dscore_h(&self, src: Pos, dest_x: u16) -> u32 {
        if dest_x < src.0 {
            (dest_x..src.0)
                .map(|x| self.cells[src.1 as usize][x as usize] as u32)
                .sum()
        } else {
            (src.0 + 1..=dest_x)
                .map(|x| self.cells[src.1 as usize][x as usize] as u32)
                .sum()
        }
    }

    fn heur(&self, p1: Pos, p2: Pos) -> u32 {
        manh(p1, p2) as u32
    }

    pub fn find_best_path_to_corner(&self, min: u16, max: u16) -> u32 {
        let eng = Engine::new(self);
        eng.find_best_path((self.size.0 - 1, self.size.1 - 1), min, max)
    }
}

pub type Pos = (u16, u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Dir {
    V,
    H,
}

impl Dir {
    fn flip(self) -> Self {
        match self {
            Dir::V => Dir::H,
            Dir::H => Dir::V,
        }
    }
}

impl From<Dir> for usize {
    fn from(value: Dir) -> Self {
        match value {
            Dir::V => 0,
            Dir::H => 1,
        }
    }
}

fn add(a: u16, b: i16) -> Option<u16> {
    if b >= 0 {
        Some(a + (b as u16))
    } else {
        a.checked_sub((-b) as u16)
    }
}

fn manh(p1: Pos, p2: Pos) -> u16 {
    p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)
}

#[derive(Debug)]
struct Pair<K, V> {
    key: K,
    value: V,
}

impl<K, V> Pair<K, V> {
    fn new(key: K, value: V) -> Self {
        Self { key, value }
    }
}

impl<K: PartialEq, V> PartialEq for Pair<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<K: Eq, V> Eq for Pair<K, V> {}

impl<K: PartialOrd, V> PartialOrd for Pair<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.key.partial_cmp(&other.key)
    }
}

impl<K: Ord, V> Ord for Pair<K, V> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.key.cmp(&other.key)
    }
}

#[derive(Debug)]
struct Engine<'a> {
    queue: Heap<Reverse<Pair<u32, (u32, Pos, Dir)>>>,
    scores: Vec<Vec<[u32; 2]>>,
    map: &'a Map,
}

impl<'a> Engine<'a> {
    fn new(map: &'a Map) -> Self {
        Self {
            queue: <_>::default(),
            scores: vec![vec![[u32::MAX, u32::MAX]; map.size.0 as usize]; map.size.1 as usize],
            map,
        }
    }

    fn find_best_path(mut self, target: Pos, min: u16, max: u16) -> u32 {
        let init_manh = self.map.heur((0, 0), target);
        self.queue
            .push(Reverse(Pair::new(init_manh, (0, (0, 0), Dir::H))));
        self.queue
            .push(Reverse(Pair::new(init_manh, (0, (0, 0), Dir::V))));
        self.scores[0][0][0] = 0;
        self.scores[0][0][1] = 0;

        while let Some(Reverse(Pair {
            key: _,
            value: (score, pos, dir),
        })) = self.queue.pop()
        {
            if score <= self.scores[pos.1 as usize][pos.0 as usize][usize::from(dir)] {
                self.scores[pos.1 as usize][pos.0 as usize][usize::from(dir)] = score;

                if pos == target {
                    break;
                }

                let flip_dir = dir.flip();

                for (dst, dscore) in self.map.positions(pos, flip_dir, min, max) {
                    let dest_score = score + dscore;
                    if dest_score
                        < self.scores[dst.1 as usize][dst.0 as usize][usize::from(flip_dir)]
                    {
                        let key = dest_score + self.map.heur(dst, target);
                        self.queue
                            .push(Reverse(Pair::new(key, (dest_score, dst, flip_dir))));
                    }
                }
            }
        }

        let target_values = self.scores[target.1 as usize][target.0 as usize];
        std::cmp::min(target_values[0], target_values[1])
    }
}
