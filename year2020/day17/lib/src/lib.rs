use itertools::Itertools as _;
use std::{collections::HashSet, io::BufRead, ops::Range};

trait Extend<T> {
    fn insert(&mut self, val: T);
    fn around(&self) -> Self;
}

impl Extend<i32> for Range<i32> {
    fn insert(&mut self, val: i32) {
        if val < self.start {
            self.start = val;
        }
        if val >= self.end {
            self.end = val + 1;
        }
    }

    fn around(&self) -> Self {
        (self.start - 1)..(self.end + 1)
    }
}

#[derive(Debug, Default)]
pub struct Scene3<T> {
    data: HashSet<(T, T, T)>,
    x: Range<T>,
    y: Range<T>,
    z: Range<T>,
}

impl Scene3<i32> {
    pub fn new<R: BufRead>(inp: R) -> Self {
        let mut sc = Self::default();
        for (y, line) in inp.lines().enumerate() {
            for (x, c) in line.unwrap().chars().enumerate() {
                if c == '#' {
                    sc.set(x as _, y as _, 0);
                }
            }
        }
        sc
    }

    fn set(&mut self, x: i32, y: i32, z: i32) {
        self.data.insert((x, y, z));
        self.x.insert(x);
        self.y.insert(y);
        self.z.insert(z);
    }

    pub fn advance(&self) -> Self {
        let mut sc = Self::default();
        for x in self.x.around() {
            for y in self.y.around() {
                for z in self.z.around() {
                    let n = nbhood3()
                        .filter(|&(dx, dy, dz)| self.data.contains(&(x + dx, y + dy, z + dz)))
                        .count();
                    if n == 3 || (self.data.contains(&(x, y, z)) && n == 2) {
                        sc.set(x, y, z);
                    }
                }
            }
        }
        sc
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

fn nbhood3() -> impl Iterator<Item = (i32, i32, i32)> {
    [-1, 0, 1]
        .into_iter()
        .cartesian_product([-1, 0, 1])
        .cartesian_product([-1, 0, 1])
        .filter_map(|((x, y), z)| {
            if x == 0 && y == 0 && z == 0 {
                None
            } else {
                Some((x, y, z))
            }
        })
}

#[derive(Debug, Default)]
pub struct Scene4<T> {
    // TODO As we need only 6 steps, one could replace the HashSet with Vec of a predictable size.
    data: HashSet<(T, T, T, T)>,
    x: Range<T>,
    y: Range<T>,
    z: Range<T>,
    t: Range<T>,
}

impl Scene4<i32> {
    pub fn new<R: BufRead>(inp: R) -> Self {
        let mut sc = Self::default();
        for (y, line) in inp.lines().enumerate() {
            for (x, c) in line.unwrap().chars().enumerate() {
                if c == '#' {
                    sc.set(x as _, y as _, 0, 0);
                }
            }
        }
        sc
    }

    fn set(&mut self, x: i32, y: i32, z: i32, t: i32) {
        self.data.insert((x, y, z, t));
        self.x.insert(x);
        self.y.insert(y);
        self.z.insert(z);
        self.t.insert(t);
    }

    pub fn advance(&self) -> Self {
        let mut sc = Self::default();
        for x in self.x.around() {
            for y in self.y.around() {
                for z in self.z.around() {
                    for t in self.t.around() {
                        let n = nbhood4()
                            .filter(|&(dx, dy, dz, dt)| {
                                self.data.contains(&(x + dx, y + dy, z + dz, t + dt))
                            })
                            .count();
                        if n == 3 || (self.data.contains(&(x, y, z, t)) && n == 2) {
                            sc.set(x, y, z, t);
                        }
                    }
                }
            }
        }
        sc
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

fn nbhood4() -> impl Iterator<Item = (i32, i32, i32, i32)> {
    [-1, 0, 1]
        .into_iter()
        .cartesian_product([-1, 0, 1])
        .cartesian_product([-1, 0, 1])
        .cartesian_product([-1, 0, 1])
        .filter_map(|(((x, y), z), t)| {
            if x == 0 && y == 0 && z == 0 && t == 0 {
                None
            } else {
                Some((x, y, z, t))
            }
        })
}
