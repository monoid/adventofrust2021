use std::{
    collections::HashSet,
    hash::Hash,
    io::{self, BufRead},
};

pub fn read_map() -> Vec<Vec<u8>> {
    let stdin = io::stdin().lock();
    stdin
        .lines()
        .map(|s| s.unwrap().chars().map(|c| (c as u8) - b'0').collect())
        .collect()
}

fn find_visibles<V, I>(it: I, store: &mut HashSet<V>)
where
    V: Hash + Eq,
    I: Iterator<Item = (u8, V)>,
{
    let mut max: Option<u8> = None;
    for (h, v) in it {
        if let Some(m) = max {
            if m < h {
                store.insert(v);
                max = Some(h);
            }
        } else {
            store.insert(v);
            max = Some(h);
        }
    }
}

pub fn count_all_visibles(map: &[Vec<u8>]) -> usize {
    let mut visibles = HashSet::<(usize, usize)>::new();

    let height = map.len();
    let width = map[0].len();
    for (y, row) in map.iter().enumerate() {
        find_visibles(
            row.iter().cloned().enumerate().map(|(x, v)| (v, (x, y))),
            &mut visibles,
        );

        find_visibles(
            row.iter()
                .cloned()
                .enumerate()
                .rev()
                .map(|(x, v)| (v, (x, y))),
            &mut visibles,
        );
    }

    for x in 0..width {
        find_visibles((0..height).map(|y| (map[y][x], (x, y))), &mut visibles);
    }

    for x in 0..width {
        find_visibles(
            (0..height).map(|y| (map[height - y - 1][x], (x, height - y - 1))),
            &mut visibles,
        );
    }
    visibles.len()
}

fn scenic_for_dir<I: Iterator<Item = u8>>(center: u8, it: I) -> usize {
    let mut cnt = 0;
    for v in it {
        cnt += 1;
        if v >= center {
            break;
        }
    }
    cnt
}

fn scenic_score(x: usize, y: usize, map: &[Vec<u8>]) -> usize {
    let center = map[y][x];
    let height = map.len();
    let width = map[0].len();

    scenic_for_dir(center, map[y][(x + 1)..width].iter().cloned())
        * scenic_for_dir(center, map[y][0..x].iter().cloned().rev())
        * scenic_for_dir(center, (0..y).map(|y1| map[y1][x]).rev())
        * scenic_for_dir(center, ((y + 1)..height).map(|y1| map[y1][x]))
}

pub fn max_scenic_score(map: &[Vec<u8>]) -> Option<usize> {
    let height = map.len();
    let width = map[0].len();

    (0..height)
        .flat_map(|y| (0..width).map(move |x| scenic_score(x, y, map)))
        .max()
}
