use std::{collections::HashMap, io};

pub fn read_data<R: io::BufRead>(inp: R) -> io::Result<Vec<String>> {
    inp.lines().collect()
}

pub fn apply_tile(map: &mut HashMap<(isize, isize), bool>, actions: &str) {
    let mut p = actions.chars();
    let mut x = 0;
    let mut y = 0;

    loop {
        match p.next() {
            Some('e') => x += 1,
            Some('w') => x -= 1,
            Some('s') => match p.next().unwrap() {
                'e' => {
                    x += (y + 1) & 1;
                    y -= 1;
                }
                'w' => {
                    x -= y & 1;
                    y -= 1;
                }
                _ => panic!("Unexpected char"),
            },
            Some('n') => match p.next().unwrap() {
                'e' => {
                    x += (y + 1) & 1;
                    y += 1;
                }
                'w' => {
                    x -= y & 1;
                    y += 1;
                }
                _ => panic!("Unexpected char"),
            },
            Some(_) => panic!("Unexpected char"),
            None => break,
        }
    }
    if let std::collections::hash_map::Entry::Vacant(e) = map.entry((x, y)) {
        e.insert(true);
    } else {
        map.remove(&(x, y));
    }
}

fn count_black(tiles: &HashMap<(isize, isize), bool>, x: isize, y: isize) -> usize {
    let mut count = 0;
    if tiles.contains_key(&(x + 1, y)) {
        count += 1;
    }
    if tiles.contains_key(&(x - 1, y)) {
        count += 1;
    }
    if tiles.contains_key(&(x + ((y + 1) & 1), y - 1)) {
        count += 1;
    }
    if tiles.contains_key(&(x - (y & 1), y - 1)) {
        count += 1;
    }
    if tiles.contains_key(&(x + ((y + 1) & 1), y + 1)) {
        count += 1;
    }
    if tiles.contains_key(&(x - (y & 1), y + 1)) {
        count += 1;
    }
    count
}

pub fn life(tiles: HashMap<(isize, isize), bool>) -> HashMap<(isize, isize), bool> {
    let minx = tiles.keys().map(|(x, _y)| *x).min().unwrap() - 1;
    let miny = tiles.keys().map(|(_x, y)| *y).min().unwrap() - 1;
    let maxx = tiles.keys().map(|(x, _y)| *x).max().unwrap() + 1;
    let maxy = tiles.keys().map(|(_x, y)| *y).max().unwrap() + 1;

    let mut res = HashMap::<(isize, isize), bool>::new();
    for x in minx..=maxx {
        for y in miny..=maxy {
            let counts = count_black(&tiles, x, y);
            let col = tiles.get(&(x, y)).cloned().unwrap_or_default();
            if col {
                if counts == 1 || counts == 2 {
                    res.insert((x, y), true);
                }
            } else if counts == 2 {
                res.insert((x, y), true);
            }
        }
    }

    res
}
