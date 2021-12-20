use std::{
    collections::HashSet,
    io::{self, BufRead as _},
};

pub type Map = HashSet<(isize, isize)>;

/// Bit pattern for pixels around (x, y)
fn n_around(x: isize, y: isize, map: &Map, neg_inp: bool) -> usize {
    (-1..=1)
        .flat_map(|dy| (-1..=1).map(move |dx| (x + dx, y + dy)))
        .map(|(nx, ny)| (map.contains(&(nx, ny)) ^ neg_inp) as usize)
        .fold(0, |acc, v| (acc << 1) | v)
}

pub fn enchance(map: Map, algo: &str, neg_inp: bool, neg_out: bool) -> Map {
    let minx = map.iter().map(|p| p.0).min().unwrap();
    let maxx = map.iter().map(|p| p.0).max().unwrap();
    let miny = map.iter().map(|p| p.1).min().unwrap();
    let maxy = map.iter().map(|p| p.1).max().unwrap();

    ((minx - 1)..=(maxx + 1))
        .flat_map(|x| ((miny - 3)..=(maxy + 3)).map(move |y| (x, y)))
        .filter_map(|(x, y)| {
            if neg_out != (algo.chars().nth(n_around(x, y, &map, neg_inp)).unwrap() == '#') {
                Some((x, y))
            } else {
                None
            }
        })
        .collect()
}

pub fn read_data() -> (String, Map) {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let mut lines = stdin.lines();

    let algo = lines.next().unwrap().unwrap();
    lines.next().unwrap().unwrap();

    let map = Map::from_iter(lines.map(Result::unwrap).enumerate().flat_map(|(y, s)| {
        s.chars()
            .collect::<Vec<_>>()
            .into_iter()
            .enumerate()
            .filter_map(move |(x, c)| {
                if c == '#' {
                    Some((x as isize, y as isize))
                } else {
                    None
                }
            })
    }));

    (algo, map)
}
