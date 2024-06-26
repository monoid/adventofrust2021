use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;
use std::str::FromStr;

use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::sequence::separated_pair;

pub type Coord = i32;
pub type XY = (Coord, Coord);
pub type Z = Coord;
pub type Pt = (XY, Z);
pub type SlabId = usize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Slab {
    item: Item,
    pub supported_by: HashSet<SlabId>,
    pub dominators: HashSet<SlabId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Item {
    X {
        x: RangeInclusive<Coord>,
        y: Coord,
        z: Coord,
    },
    Y {
        x: Coord,
        y: RangeInclusive<Coord>,
        z: Coord,
    },
    Z {
        x: Coord,
        y: Coord,
        z: RangeInclusive<Coord>,
    },
}

impl Item {
    fn new(a1: Pt, a2: Pt) -> Self {
        if a1.0 .0 == a2.0 .0 {
            let cx = a1.0 .0;
            if a1.0 .1 == a2.0 .1 {
                let cy = a1.0 .1;
                let min_cz = std::cmp::min(a1.1, a2.1);
                let max_cz = std::cmp::max(a1.1, a2.1);
                return Item::Z {
                    x: cx,
                    y: cy,
                    z: min_cz..=max_cz,
                };
            }
            // else
            if a1.1 == a2.1 {
                let cz = a1.1;
                let min_cy = std::cmp::min(a1.0 .1, a2.0 .1);
                let max_cy = std::cmp::max(a1.0 .1, a2.0 .1);

                return Item::Y {
                    x: cx,
                    y: min_cy..=max_cy,
                    z: cz,
                };
            }
        } else if a1.0 .1 == a2.0 .1 && a1.1 == a2.1 {
            let cy = a1.0 .1;
            let cz = a1.1;
            let min_cx = std::cmp::min(a1.0 .0, a2.0 .0);
            let max_cx = std::cmp::max(a1.0 .0, a2.0 .0);

            return Item::X {
                x: min_cx..=max_cx,
                y: cy,
                z: cz,
            };
        }

        panic!("only linear features are supported")
    }
}

impl Iterator for Item {
    type Item = Pt;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Item::X { x, y, z } => x.next().map(|x| ((x, *y), *z)),
            Item::Y { x, y, z } => y.next().map(|y| ((*x, y), *z)),
            Item::Z { x, y, z } => z.next().map(|z| ((*x, *y), z)),
        }
    }
}

impl Slab {
    pub fn new(a1: Pt, a2: Pt) -> Self {
        Self {
            item: Item::new(a1, a2),
            supported_by: <_>::default(),
            dominators: <_>::default(),
        }
    }

    pub fn min_z(&self) -> i32 {
        match &self.item {
            Item::X { z, .. } => *z,
            Item::Y { z, .. } => *z,
            Item::Z { z, .. } => *z.start(),
        }
    }

    pub fn upper_face(&self) -> impl Iterator<Item = Pt> {
        match &self.item {
            Item::X { .. } | Item::Y { .. } => self.item.clone(),
            Item::Z { x, y, z } => Item::Z {
                x: *x,
                y: *y,
                z: *z.end()..=*z.end(),
            },
        }
    }

    pub fn lower_face(&self) -> impl Iterator<Item = Pt> {
        match &self.item {
            Item::X { .. } | Item::Y { .. } => self.item.clone(),
            Item::Z { x, y, z } => Item::Z {
                x: *x,
                y: *y,
                z: *z.start()..=*z.start(),
            },
        }
    }
}

impl FromStr for Slab {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use nom::combinator::all_consuming;
        all_consuming(parse_slab)(s)
            .map(|(_, v)| v)
            .map_err(|e| e.to_string())
    }
}

fn parse_pt(inp: &str) -> nom::IResult<&str, Pt> {
    use nom::character::complete::i32 as parse_i32;
    separated_pair(
        separated_pair(parse_i32, tag(","), parse_i32),
        tag(","),
        parse_i32,
    )(inp)
}

fn parse_slab(inp: &str) -> nom::IResult<&str, Slab> {
    map(separated_pair(parse_pt, tag("~"), parse_pt), |(a1, a2)| {
        Slab::new(a1, a2)
    })(inp)
}

pub fn read_data() -> Vec<Slab> {
    std::io::stdin()
        .lines()
        .map(|l| l.unwrap().trim().parse().unwrap())
        .collect()
}

#[derive(Default, Debug)]
pub struct Map {
    pub map: HashMap<XY, (SlabId, Z)>,
}

impl Map {
    pub fn new() -> Self {
        <_>::default()
    }

    pub fn drop_slap(&mut self, slab_id: SlabId, slab: &mut Slab) {
        let mut h: Z = 0; // ground

        // collect heights and slab.supported_by
        for (xy, _) in slab.lower_face() {
            if let Some(&(other_id, other_z)) = self.map.get(&xy) {
                use std::cmp::Ordering;

                match h.cmp(&other_z) {
                    Ordering::Less => {
                        // new height! Reset
                        h = other_z;
                        slab.supported_by.clear();
                        slab.supported_by.insert(other_id);
                    }
                    Ordering::Equal => {
                        slab.supported_by.insert(other_id);
                    }
                    Ordering::Greater => {}
                }
            }
        }

        assert!(slab.min_z() > h, "incorrect data or ordering");

        let min_z = slab.min_z();
        let fall_height = min_z - h - 1;
        // update the slab
        match &mut slab.item {
            Item::X { z, .. } => {
                *z -= fall_height;
            }
            Item::Y { z, .. } => {
                *z -= fall_height;
            }
            Item::Z { z, .. } => {
                *z = (z.start() - fall_height)..=(z.end() - fall_height);
            }
        }

        // update map
        // please note that vertical slabs starts from min_z to max_z which is last;
        // we do here extra work, but it is fast to write
        for (xy, z) in slab.upper_face() {
            self.map.insert(xy, (slab_id, z));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter_point() {
        let slab = Slab::new(((0, 0), 0), ((0, 0), 0));
        let pts: Vec<_> = slab.upper_face().collect();
        assert_eq!(pts, vec![((0, 0), 0)]);
    }

    #[test]
    fn test_iter_x1() {
        let slab = Slab::new(((0, 0), 0), ((2, 0), 0));
        let pts: Vec<_> = slab.upper_face().collect();
        assert_eq!(pts, vec![((0, 0), 0), ((1, 0), 0), ((2, 0), 0)]);
    }

    #[test]
    fn test_iter_x2() {
        let slab = Slab::new(((2, 0), 0), ((0, 0), 0));
        let pts: Vec<_> = slab.upper_face().collect();
        assert_eq!(pts, vec![((0, 0), 0), ((1, 0), 0), ((2, 0), 0)]);
    }

    #[test]
    fn test_iter_y1() {
        let slab = Slab::new(((0, 0), 0), ((0, 2), 0));
        let pts: Vec<_> = slab.upper_face().collect();
        assert_eq!(pts, vec![((0, 0), 0), ((0, 1), 0), ((0, 2), 0)]);
    }

    #[test]
    fn test_iter_y2() {
        let slab = Slab::new(((0, 2), 0), ((0, 0), 0));
        let pts: Vec<_> = slab.upper_face().collect();
        assert_eq!(pts, vec![((0, 0), 0), ((0, 1), 0), ((0, 2), 0)]);
    }

    #[test]
    fn test_iter_lower_z1() {
        let slab = Slab::new(((0, 0), 0), ((0, 0), 2));
        let pts: Vec<_> = slab.lower_face().collect();
        assert_eq!(pts, vec![((0, 0), 0)]);
    }

    #[test]
    fn test_iter_lower_z2() {
        let slab = Slab::new(((0, 0), 2), ((0, 0), 0));
        let pts: Vec<_> = slab.lower_face().collect();
        assert_eq!(pts, vec![((0, 0), 0)]);
    }

    #[test]
    fn test_iter_upper_z1() {
        let slab = Slab::new(((0, 0), 0), ((0, 0), 2));
        let pts: Vec<_> = slab.upper_face().collect();
        assert_eq!(pts, vec![((0, 0), 2)]);
    }

    #[test]
    fn test_iter_upper_z2() {
        let slab = Slab::new(((0, 0), 2), ((0, 0), 0));
        let pts: Vec<_> = slab.upper_face().collect();
        assert_eq!(pts, vec![((0, 0), 2)]);
    }
}
