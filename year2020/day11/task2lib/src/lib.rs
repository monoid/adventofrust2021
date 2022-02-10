use std::io;

#[derive(PartialEq, Clone)]
pub struct FlatMap<V> {
    width: usize,
    height: usize,
    data: Vec<V>,
}

impl<T: Clone> FlatMap<T> {
    fn from_iter<I: Iterator<Item = T>>(width: usize, height: usize, it: I) -> Self {
        let data = it.collect::<Vec<_>>();
        assert!(data.len() == width * height);
        Self {
            width,
            height,
            data,
        }
    }

    pub fn from_2d(data: &[Vec<T>]) -> Self {
        let height = data.len();
        let width = data[0].len();
        let res = Self::from_iter(
            width,
            height,
            data.iter().flat_map(|line| line.iter().cloned()),
        );
        assert!(res.data.len() == width * height);
        res
    }

    fn index(&self, y: isize, x: isize) -> Option<usize> {
        if (0..(self.width as isize)).contains(&x) && (0..(self.height as isize)).contains(&y) {
            Some((y as usize) * self.width + (x as usize))
        } else {
            None
        }
    }
}

impl<V> std::ops::Deref for FlatMap<V> {
    type Target = [V];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<V> std::ops::DerefMut for FlatMap<V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

type Map = FlatMap<lib::State>;

fn visibility(map: &Map, y: usize, x: usize) -> arrayvec::ArrayVec<u16, 8> {
    [
        (0isize, 1isize),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ]
    .into_iter()
    .filter_map(|(dy, dx)| {
        let mut sy = y as isize + dy;
        let mut sx = x as isize + dx;

        while let Some(pos) = map.index(sy as _, sx as _) {
            if map[pos] == lib::State::Floor {
                sy += dy;
                sx += dx;
            } else {
                return Some(pos as _);
            }
        }
        None
    })
    .collect()
}

pub fn produce_visibility_map(map: &Map) -> Vec<(arrayvec::ArrayVec<u16, 8>, usize)> {
    (0..map.height)
            .flat_map(|y| (0..map.width).map(move |x| (y, x)))
            .filter_map(|(y, x)| {
                let ind = map.index(y as _, x as _).unwrap();
                if map[ind] == lib::State::Floor {
                    None
                } else {
                    Some((visibility(map, y, x), ind))
                }
            }).collect()
}

fn next_map(map: &Map, into: &mut Map, vis_map: &[(arrayvec::ArrayVec<u16, 8>, usize)]) -> bool {
    let mut changed = false;
    into.data.fill(lib::State::Floor);

    for (vis, pos) in vis_map {
        let occ = vis.iter().filter(|&&pos| map[pos as usize] == lib::State::Occupied).count();
        match map[*pos] {
            lib::State::Occupied => {
                into[*pos] = if occ >= 5 {
                    changed = true;
                    lib::State::Empty
                } else {
                    lib::State::Occupied
                };
            }
            lib::State::Empty => {
                into[*pos] = if occ == 0 {
                    changed = true;
                    lib::State::Occupied
                } else {
                    lib::State::Empty
                };
            }
            lib::State::Floor => loop {}
        }
    }
    changed
}

pub fn solve(map: &lib::Map) -> usize {
    let mut map = FlatMap::from_2d(map);
    let mut map2 = map.clone();
    let vis_map = produce_visibility_map(&map);

    while next_map(&map, &mut map2, &vis_map) {
        std::mem::swap(&mut map, &mut map2);
    }
    map.iter()
       .filter(|&&state| state == lib::State::Occupied)
       .count()
}

pub fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let map = lib::read_map(stdin);
    println!("{}", solve(&map));
}
