use std::{
    collections::VecDeque,
    io::{self, BufRead as _},
};

type Map = Vec<Vec<u8>>;

fn indices_around(x: usize, y: usize, w: usize, h: usize) -> impl Iterator<Item = (usize, usize)> {
    [(-1isize, 0isize), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .filter_map(move |(dx, dy)| {
            let x1 = (x as isize) + dx;
            let y1 = (y as isize) + dy;
            if (x1 < 0) || (x1 >= (w as isize)) || (y1 < 0) || (y1 >= (h as isize)) {
                None
            } else {
                Some((x1 as usize, y1 as usize))
            }
        })
}

fn wrap(v: usize) -> u8 {
    ((v - 1) % 9 + 1) as _
}

fn full_map(map: Map) -> Map {
    let h = map.len();
    let w = map[0].len();

    (0..5 * h)
        .map(|y| {
            (0..5 * w)
                .map(|x| wrap(map[y % h][x % w] as usize + (y / h) + (x / w)))
                .collect()
        })
        .collect()
}

struct Sample {
    cnt: usize,
    each: usize,
}

impl Sample {
    fn new(each: usize) -> Self {
        Self { cnt: 0, each }
    }

    fn sample<D: std::fmt::Debug>(&mut self, v: &D) {
        self.cnt += 1;
        if self.cnt >= self.each {
            self.cnt = 0;
            eprintln!("sample: {:?}", v);
        }
    }
}

fn main() {
    let map: Map = io::stdin()
        .lock()
        .lines()
        .map(|s| s.unwrap().chars().map(|c| c as u8 - b'0').collect())
        .collect();
    let map = full_map(map);

    let mut sample = Sample::new(1000);

    let h = map.len();
    let w = map[0].len();
    let mut search = vec![vec![u32::MAX; w]; h];
    search[0][0] = 0;
    let mut qju = VecDeque::new();
    qju.push_back((0, 0));

    let mut max_dest = u32::MAX;

    while let Some((x, y)) = qju.pop_front() {
        let c = search[y][x];
        if (x, y) == (w - 1, h - 1) {
            max_dest = std::cmp::min(max_dest, c);
            sample.sample(&(max_dest, qju.len()));
        }
        for (x1, y1) in indices_around(x, y, w, h) {
            let new = c + (map[y1][x1] as u32);
            if search[y1][x1] > new {
                search[y1][x1] = new;
                if new < max_dest {
                    qju.push_back((x1, y1));
                }
            }
        }
    }

    println!("{}", search[h - 1][w - 1]);
}
