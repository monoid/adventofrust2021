use itertools::{Either, Itertools as _};
use std::ops::RangeInclusive;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Cuboid {
    pub x: RangeInclusive<i32>,
    pub y: RangeInclusive<i32>,
    pub z: RangeInclusive<i32>,
}

#[inline]
fn intersect1<T: Ord + Clone>(
    r1: &RangeInclusive<T>,
    r2: &RangeInclusive<T>,
) -> Option<RangeInclusive<T>> {
    let s = std::cmp::max(r1.start(), r2.start()).clone();
    let e = std::cmp::min(r1.end(), r2.end()).clone();
    if s > e {
        None
    } else {
        Some(s..=e)
    }
}

#[inline]
fn contains1<T: Ord + Clone>(r1: &RangeInclusive<T>, r2: &RangeInclusive<T>) -> bool {
    r1.contains(r2.start()) && r1.contains(r2.end())
}

impl Cuboid {
    pub fn from_coord(x1: i32, x2: i32, y1: i32, y2: i32, z1: i32, z2: i32) -> Self {
        Self {
            x: x1..=x2,
            y: y1..=y2,
            z: z1..=z2,
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.x.is_empty() || self.y.is_empty() || self.z.is_empty()
    }

    #[inline]
    pub fn volume(&self) -> usize {
        (self.x.end() - self.x.start() + 1) as usize
            * (self.y.end() - self.y.start() + 1) as usize
            * (self.z.end() - self.z.start() + 1) as usize
    }

    #[inline]
    pub fn intersect(&self, other: &Self) -> Option<Self> {
        // I hope the compiler will manager to inline and optimize
        // this code.
        intersect1(&self.x, &other.x)
            .zip(intersect1(&self.y, &other.y))
            .zip(intersect1(&self.z, &other.z))
            .map(|((x, y), z)| Self { x, y, z })
    }

    pub fn is_intersecting(&self, other: &Self) -> bool {
        // I hope the compiler will manager to inline and optimize
        // this code.
        self.intersect(other).is_some()
    }

    pub fn contains(&self, other: &Self) -> bool {
        contains1(&self.x, &other.x) && contains1(&self.y, &other.y) && contains1(&self.z, &other.z)
    }

    // Subtract one cuboid from the other, returinig collection of resulting cuboids.
    pub fn subtract(&self, other: &Self) -> impl Iterator<Item = Self> {
        let inter = self.intersect(other);
        match inter {
            // Return itself unmodified
            None => Either::Left(std::iter::once(self.clone())),
            Some(inter) => {
                Either::Right(if other.contains(self) {
                    // Complete substraction
                    Either::Left(std::iter::empty())
                } else {
                    // Now the hardest part.
                    let subdivisions = [
                        *self.x.start()..=*inter.x.start() - 1,
                        *inter.x.start()..=*inter.x.end(),
                        *inter.x.end() + 1..=*self.x.end(),
                    ]
                    .into_iter()
                    .enumerate()
                    .filter(|(_, x)| !x.is_empty())
                    .cartesian_product(
                        [
                            *self.y.start()..=*inter.y.start() - 1,
                            *inter.y.start()..=*inter.y.end(),
                            *inter.y.end() + 1..=*self.y.end(),
                        ]
                        .into_iter()
                        .enumerate()
                        .filter(|(_, y)| !y.is_empty()),
                    )
                    .cartesian_product(
                        [
                            *self.z.start()..=*inter.z.start() - 1,
                            *inter.z.start()..=*inter.z.end(),
                            *inter.z.end() + 1..=*self.z.end(),
                        ]
                        .into_iter()
                        .enumerate()
                        .filter(|(_, z)| !z.is_empty()),
                    )
                    .filter_map(|(((ix, x), (iy, y)), (iz, z))| {
                        if ix == 1 && iy == 1 && iz == 1 {
                            // Skip the "central" hole
                            None
                        } else {
                            Some(Cuboid { x, y, z })
                        }
                    });
                    Either::Right(subdivisions)
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_non_intersect() {
        let c1 = Cuboid::from_coord(-10, 0, -20, 0, -30, 0);
        let c2 = Cuboid::from_coord(2, 10, 2, 10, 2, 10);
        assert_eq!(c1.intersect(&c2), None);
    }

    #[test]
    fn test_intersect1() {
        let c1 = Cuboid::from_coord(-10, 0, -20, 0, -30, 0);
        let c2 = Cuboid::from_coord(0, 10, 0, 10, 0, 10);
        assert_eq!(
            c1.intersect(&c2),
            Some(Cuboid::from_coord(0, 0, 0, 0, 0, 0))
        );
    }

    #[test]
    fn test_intersect_equal() {
        let c1 = Cuboid::from_coord(-10, 0, -20, 0, -30, 0);
        assert_eq!(c1.intersect(&c1), Some(c1));
    }

    #[test]
    fn test_non_contain() {
        let c1 = Cuboid::from_coord(-10, 0, -20, 0, -30, 0);
        let c2 = Cuboid::from_coord(2, 10, 2, 10, 2, 10);
        assert!(!c1.contains(&c2));
    }

    #[test]
    fn test_non_contain_intersect1() {
        let c1 = Cuboid::from_coord(-10, 0, -20, 0, -30, 0);
        let c2 = Cuboid::from_coord(0, 10, 0, 10, 0, 10);
        assert!(!c1.contains(&c2));
    }

    #[test]
    fn test_contains() {
        let c1 = Cuboid::from_coord(0, 20, 0, 20, 0, 20);
        let c2 = Cuboid::from_coord(0, 10, 2, 20, 2, 10);
        assert!(c1.contains(&c2));
    }

    #[test]
    fn test_contains_equal() {
        let c1 = Cuboid::from_coord(-10, 0, -20, 0, -30, 0);
        assert!(c1.contains(&c1));
    }

    #[test]
    fn test_volume1() {
        assert_eq!(Cuboid::from_coord(0, 0, 0, 0, 0, 0).volume(), 1);
    }

    #[test]
    fn test_subtract_nonintersect() {
        let c1 = Cuboid::from_coord(-10, 0, -20, 0, -30, 0);
        let c2 = Cuboid::from_coord(2, 10, 2, 10, 2, 10);
        let v1 = c1.subtract(&c2).collect_vec();
        assert_eq!(v1, vec![c1]);
    }

    #[test]
    fn test_subtract_self() {
        let c1 = Cuboid::from_coord(-10, 0, -20, 0, -30, 0);
        let v1 = c1.subtract(&c1).collect_vec();
        assert_eq!(v1, vec![]);
    }

    #[test]
    fn test_subtract_larger() {
        let c1 = Cuboid::from_coord(0, 10, 0, 10, 0, 10);
        let c2 = Cuboid::from_coord(-20, 20, -20, 20, -20, 20);
        let v1 = c1.subtract(&c2).collect_vec();
        assert_eq!(v1, vec![]);
    }

    #[test]
    fn test_subtract_contained() {
        let c1 = Cuboid::from_coord(-20, 20, -20, 20, -20, 20);
        let c2 = Cuboid::from_coord(0, 10, 0, 10, 0, 10);
        let vol = c1.subtract(&c2).map(|c| c.volume()).sum::<usize>();
        assert_eq!(vol, c1.volume() - c2.volume());
    }

    #[test]
    fn test_subtract_top() {
        let c1 = Cuboid::from_coord(-20, 20, -20, 20, -20, 20);
        let c2 = Cuboid::from_coord(0, 20, 0, 10, 0, 10);
        let vol = c1.subtract(&c2).map(|c| c.volume()).sum::<usize>();
        assert_eq!(vol, c1.volume() - c2.volume());
    }

    #[test]
    fn test_subtract_bottom() {
        let c1 = Cuboid::from_coord(-20, 20, -20, 20, -20, 20);
        let c2 = Cuboid::from_coord(-20, 10, 0, 10, 0, 10);
        let vol = c1.subtract(&c2).map(|c| c.volume()).sum::<usize>();
        assert_eq!(vol, c1.volume() - c2.volume());
    }

    #[test]
    fn test_subtract_left() {
        let c1 = Cuboid::from_coord(-20, 20, -20, 20, -20, 20);
        let c2 = Cuboid::from_coord(-20, 10, 0, 10, 0, 10);
        let vol = c1.subtract(&c2).map(|c| c.volume()).sum::<usize>();
        assert_eq!(vol, c1.volume() - c2.volume());
    }
}
