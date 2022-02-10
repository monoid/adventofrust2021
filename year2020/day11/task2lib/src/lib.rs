use std::io;

#[derive(PartialEq, Clone)]
struct FlatMap<V> {
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

    fn from_2d(data: Vec<Vec<T>>) -> Self {
        let height = data.len();
        let width = data[0].len();
        let res = Self::from_iter(
            width,
            height,
            data.into_iter().flat_map(|line| line.into_iter()),
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

fn visibility(map: &Map, y: usize, x: usize) -> Vec<usize> {
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
                return Some(pos);
            }
        }
        None
    })
    .collect()
}

fn produce_visibility_map(map: &Map) -> FlatMap<Vec<usize>> {
    FlatMap::from_iter(
        map.width,
        map.height,
        (0..map.height).flat_map(|y| (0..map.width).map(move |x| visibility(map, y, x))),
    )
}

fn next_map(map: &Map, vis_map: &FlatMap<Vec<usize>>) -> Map {
    Map::from_iter(
        map.width,
        map.height,
        map.iter().zip(vis_map.iter()).map(|(&c, vis)| match c {
            lib::State::Floor => c,
            lib::State::Occupied => {
                let occ = vis
                    .iter()
                    .filter(|&&pos| map[pos] == lib::State::Occupied)
                    .count();
                if occ >= 5 {
                    lib::State::Empty
                } else {
                    c
                }
            }
            lib::State::Empty => {
                let occ = vis
                    .iter()
                    .filter(|&&pos| map[pos] == lib::State::Occupied)
                    .count();
                if occ == 0 {
                    lib::State::Occupied
                } else {
                    c
                }
            }
        }),
    )
}

pub fn solve(map: lib::Map) -> usize {
    let mut map = FlatMap::from_2d(map);
    let vis_map = produce_visibility_map(&map);

    loop {
        let new_map = next_map(&map, &vis_map);

        if map == new_map {
            break;
        } else {
            map = new_map;
        }
    }
    map.iter()
       .filter(|&&state| state == lib::State::Occupied)
       .count()
}

pub fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let map = lib::read_map(stdin);
    println!("{}", solve(map));
}
