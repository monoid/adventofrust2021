use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

pub struct Map {
    pub stations: HashMap<char, Vec<(isize, isize)>>,
    pub width: isize,
    pub height: isize,
}

impl Map {
    pub fn read_map<R: BufRead>(inp: R) -> Map {
        let mut stations = HashMap::<char, Vec<_>>::new();
        let mut width = 0;
        let mut height = 0;

        for (y, line) in inp.lines().enumerate() {
            let line = line.unwrap();
            width = line.len() as isize;
            height = y as isize;
            for (x, c) in line.chars().enumerate() {
                if c.is_alphanumeric() {
                    let entry = stations.entry(c).or_default();
                    entry.push((x as isize, y as isize));
                }
            }
        }
        height += 1;

        Map {
            stations,
            width,
            height,
        }
    }

    pub fn count_spots1(&self) -> usize {
        let mut spots = HashSet::new();

        for stations in self.stations.values() {
            for (a, b) in stations.iter().cloned().tuple_combinations() {
                let value = spot(a, b);
                if is_valid(value, self.width, self.height) {
                    spots.insert(value);
                }
                let value = spot(b, a);
                if is_valid(value, self.width, self.height) {
                    spots.insert(value);
                }
            }
        }

        spots.len()
    }

    pub fn count_spots2(&self) -> usize {
        let mut items = HashSet::new();

        for stations in self.stations.values() {
            for (a, b) in stations.iter().cloned().tuple_combinations() {
                let spots = all_spot(a, b);
                for value in spots.take_while(|&value| is_valid(value, self.width, self.height)) {
                    items.insert(value);
                }
                let spots = all_spot(b, a);
                for value in spots.take_while(|&value| is_valid(value, self.width, self.height)) {
                    items.insert(value);
                }
            }
        }

        items.len()
    }
}

// a -> b; d = b - a; c= a - d = a- (b - a) = 2a - b
fn spot(a: (isize, isize), b: (isize, isize)) -> (isize, isize) {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    (a.0 - dx, a.1 - dy)
}

fn all_spot(a: (isize, isize), b: (isize, isize)) -> impl Iterator<Item = (isize, isize)> {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    (1..).map(move |n| (a.0 + n * dx, a.1 + n * dy))
}

// fits in the map area
fn is_valid(a: (isize, isize), width: isize, height: isize) -> bool {
    a.0 >= 0 && a.0 < width && a.1 >= 0 && a.1 < height
}
