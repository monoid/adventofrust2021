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

fn main() {
    let map: Map = io::stdin()
        .lock()
        .lines()
        .map(|s| s.unwrap().chars().map(|c| c as u8 - b'0').collect())
        .collect();
    let h = map.len();
    let w = map[0].len();
    let mut search = vec![vec![u32::MAX; w]; h];
    search[0][0] = 0;
    let mut qju = VecDeque::new();
    qju.push_front((0, 0));

    while let Some((x, y)) = qju.pop_back() {
        for (x1, y1) in indices_around(x, y, w, h) {
            let new = search[y][x] + (map[y1][x1] as u32);
            if search[y1][x1] > new {
                search[y1][x1] = new;
                qju.push_back((x1, y1));
            }
        }
    }

    println!("{}", search[h - 1][w - 1]);
}
