use std::{
    cmp::Reverse,
    collections::HashMap,
    io::{self, BufRead as _},
};

#[derive(Clone, Copy, PartialEq)]
enum State {
    Unknown,
    InProcess,
    Max, // Out of pool
    // We may save lot of space if we use u16 or even u8, as the field is 100x100.
    OfPool(usize, usize),
}

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

fn find_pool_for_point<V: AsMut<[State]> + AsRef<[State]>, T: AsRef<[i8]>>(
    states: &mut [V],
    map: &[T],
    x: usize,
    y: usize,
    w: usize,
    h: usize,
) -> State {
    let c = map[y].as_ref()[x];
    let mut the_state = states[y].as_mut()[x];

    if the_state != State::Unknown {
        // We do not check for the InProcess, as these nodes are excluded
        // in the else branch.
        return the_state;
    } else if c == 9 {
        the_state = State::Max;
    } else if (indices_around(x, y, w, h)).all(|(x1, y1)| map[y1].as_ref()[x1] > c) {
        the_state = State::OfPool(x, y);
    } else {
        states[y].as_mut()[x] = State::InProcess;

        // We do not check that (x, yn) has lower height, as otherwise
        // the "all other locations will always be part of exactly one basin"
        // is violated.
        let (xn, yn, _) = indices_around(x, y, w, h)
            .map(|(x1, y1)| (x1, y1, map[y1].as_ref()[x1]))
            .filter(|(x1, y1, _)| states[*y1].as_ref()[*x1] != State::InProcess)
            .min_by_key(|(_, _, v)| *v)
            .unwrap();

        the_state = find_pool_for_point(states, map, xn, yn, w, h);
    }
    states[y].as_mut()[x] = the_state;
    the_state
}

fn main() {
    let map: Vec<Vec<i8>> = io::stdin()
        .lock()
        .lines()
        .map(|lin| {
            lin.unwrap()
                .chars()
                .map(|c| c as i8 - ('0' as i8))
                .collect()
        })
        .collect();
    let h = map.len();
    let w = map[0].len();

    let mut states = vec![vec![State::Unknown; w]; h];

    for y in 0..h {
        for x in 0..w {
            find_pool_for_point(&mut states, &map, x, y, w, h);
        }
    }

    let mut counts = HashMap::<_, usize>::new();

    for row in &states {
        for x in row {
            match x {
                State::Unknown => panic!("Unknown found"),
                State::InProcess => panic!("InPorcess found"),
                State::Max => {}
                State::OfPool(px, py) => {
                    *counts.entry((px, py)).or_default() += 1;
                }
            }
        }
    }

    let mut basin_counts: Vec<_> = counts.values().cloned().collect();
    basin_counts.sort_by_key(|v| Reverse(*v));

    println!("{}", basin_counts.iter().take(3).product::<usize>())
}
