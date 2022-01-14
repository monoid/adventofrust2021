use std::{
    cmp::Ordering,
    collections::VecDeque,
    io::{self, BufRead},
};

type Cell = u8;

fn adjacent(x: usize, y: usize, w: usize, h: usize) -> impl Iterator<Item = (usize, usize)> {
    let x = x as isize;
    let y = y as isize;
    let w = w as isize;
    let h = h as isize;

    [
        (-1isize, -1isize),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
    ]
    .into_iter()
    .filter_map(move |(dx, dy)| {
        let x1 = x + dx;
        let y1 = y + dy;

        if x1 < 0 || y1 < 0 || x1 >= w || y1 >= h {
            None
        } else {
            Some((x1 as _, y1 as _))
        }
    })
}

const EXPLODE: u8 = 10;

fn advance<T: AsMut<[Cell]> + AsRef<[Cell]>>(field: &mut [T]) -> u32 {
    let mut cnt = 0;
    let h = field.len();
    let w = field[0].as_ref().len();

    let mut qju = VecDeque::<(usize, usize)>::new();
    qju.extend((0..h).flat_map(move |y| (0..w).map(move |x| (x, y))));

    while let Some((x, y)) = qju.pop_front() {
        let r = &mut field[y].as_mut()[x];
        *r += 1;
        match (*r).cmp(&EXPLODE) {
            Ordering::Less => {}
            Ordering::Equal => {
                // Transitioned from non-exploded to exploded state.
                cnt += 1;
                qju.extend(adjacent(x, y, w, h));
            }
            Ordering::Greater => {
                // Has already exploded.
                *r = EXPLODE;
            }
        }
    }
    // Reset exploded.
    for line in field.iter_mut() {
        for r in line.as_mut() {
            if *r == EXPLODE {
                *r = 0;
            }
        }
    }
    cnt
}

fn main() {
    let mut field: Vec<Vec<u8>> = io::stdin()
        .lock()
        .lines()
        .map(|s| s.unwrap().chars().map(|c| (c as u8) - b'0').collect())
        .collect();
    let mut cnt = 0;
    for _ in 0..100 {
        let a = advance(&mut field);
        cnt += a;
    }
    println!("{}", cnt);
}
