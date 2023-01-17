use std::collections::VecDeque;

pub struct Map {
    pub width: usize,
    pub height: usize,
    pub map: Vec<Vec<u8>>,
    pub visited: Vec<Vec<usize>>,
    pub queue: VecDeque<(usize, (usize, usize))>,
    pub end: (usize, usize),
}

fn altitude(val: u8) -> u8 {
    match val {
        b'S' => b'a',
        b'E' => b'z',
        val => val,
    }
}

fn can_move(src_height: u8, dst_height: u8) -> bool {
    src_height + 1 >= dst_height
}

impl Map {
    pub fn read1() -> Self {
        use std::io;

        let map: Vec<_> = io::stdin()
            .lines()
            .map(|r| r.unwrap().into_bytes())
            .collect();
        let width = map[0].len();
        let height = map.len();

        let visited = vec![vec![usize::MAX; width]; height];

        let mut start = None;
        let mut end = None;
        for (y, line) in map.iter().enumerate() {
            for (x, v) in line.iter().cloned().enumerate() {
                if v == b'S' {
                    start = Some((0, (x, y)));
                } else if v == b'E' {
                    end = Some((x, y));
                }
            }
        }

        Self {
            width,
            height,
            map,
            visited,
            queue: start.into_iter().collect(),
            end: end.expect("No end point found"),
        }
    }

    pub fn read2() -> Self {
        let mut val = Self::read1();
        val.queue.clear();
        for y in 0..val.height {
            for x in 0..val.width {
                if altitude(val.map[y][x]) == b'a' {
                    val.queue.push_back((0, (x, y)));
                }
            }
        }
        val
    }

    pub fn search(&mut self) -> usize {
        while let Some((path_len, p)) = self.queue.pop_front() {
            let moves = self.possible_moves(p, path_len + 1);
            for (len, p1) in moves {
                self.visited[p1.1][p1.0] = len;
                self.queue.push_back((len, p1));
            }
        }
        self.visited[self.end.1][self.end.0]
    }

    fn possible_moves(&self, p: (usize, usize), path_len: usize) -> Vec<(usize, (usize, usize))> {
        let val = self.map[p.1][p.0];
        if val == b'E' {
            // Final point
            return vec![];
        }
        let src_alt = altitude(val);

        let p = (p.0 as isize, p.1 as isize);
        let width = self.width as isize;
        let height = self.height as isize;

        [(-1isize, 0isize), (0, -1), (1, 0), (0, 1)]
            .into_iter()
            .filter_map(|(dx, dy)| {
                let x = p.0 + dx;
                let y = p.1 + dy;
                if (0..width).contains(&x) && (0..height).contains(&y) {
                    let x = x as usize;
                    let y = y as usize;
                    let dst_alt = altitude(self.map[y][x]);
                    if can_move(src_alt, dst_alt) && self.visited[y][x] > path_len {
                        Some((path_len, (x, y)))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    }
}
