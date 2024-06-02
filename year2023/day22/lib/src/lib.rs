use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::sequence::separated_pair;

pub type XY = (i32, i32);
pub type Z = i32;
pub type Pt = (XY, Z);
pub type SlabId = usize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Slab {
    a1: Pt,
    a2: Pt,
    pub supported_by: HashSet<SlabId>,
    pub dominators: HashSet<SlabId>,
}

impl Slab {
    pub fn new(a1: Pt, a2: Pt) -> Self {
        Self {
            a1,
            a2,
            supported_by: <_>::default(),
            dominators: <_>::default(),
        }
    }

    pub fn inner_area(&self) -> u32 {
        (self.a1.0 .0.abs_diff(self.a2.0 .0))
            * (self.a1.0 .1.abs_diff(self.a2.0 .1))
            * (self.a1.1.abs_diff(self.a2.1))
    }

    pub fn min_z(&self) -> i32 {
        std::cmp::min(self.a1.1, self.a2.1)
    }

    pub fn points(&self) -> Box<dyn Iterator<Item = Pt>> {
        if self.a1.0 .0 == self.a2.0 .0 {
            let cx = self.a1.0 .0;
            if self.a1.0 .1 == self.a2.0 .1 {
                let cy = self.a1.0 .1;
                let min_cz = std::cmp::min(self.a1.1, self.a2.1);
                let max_cz = std::cmp::max(self.a1.1, self.a2.1);
                return Box::new((min_cz..=max_cz).map(move |z| ((cx, cy), z))) as _;
            }
            // else
            if self.a1.1 == self.a2.1 {
                let cz = self.a1.1;
                let min_cy = std::cmp::min(self.a1.0 .1, self.a2.0 .1);
                let max_cy = std::cmp::max(self.a1.0 .1, self.a2.0 .1);
                return Box::new((min_cy..=max_cy).map(move |y| ((cx, y), cz))) as _;
            }
        } else if self.a1.0 .1 == self.a2.0 .1 && self.a1.1 == self.a2.1 {
            let cy = self.a1.0 .1;
            let cz = self.a1.1;
            let min_cx = std::cmp::min(self.a1.0 .0, self.a2.0 .0);
            let max_cx = std::cmp::max(self.a1.0 .0, self.a2.0 .0);

            return Box::new((min_cx..=max_cx).map(move |x| ((x, cy), cz))) as _;
        }

        panic!("only linear features are supported")
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
        for (xy, _) in slab.points() {
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
        slab.a1.1 -= fall_height;
        slab.a2.1 -= fall_height;

        // update map
        // please note that vertical slabs starts from min_z to max_z which is last;
        // we do here extra work, but it is fast to write
        for (xy, z) in slab.points() {
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
        let pts: Vec<_> = slab.points().collect();
        assert_eq!(pts, vec![((0, 0), 0)]);
    }

    #[test]
    fn test_iter_x1() {
        let slab = Slab::new(((0, 0), 0), ((2, 0), 0));
        let pts: Vec<_> = slab.points().collect();
        assert_eq!(pts, vec![((0, 0), 0), ((1, 0), 0), ((2, 0), 0)]);
    }

    #[test]
    fn test_iter_x2() {
        let slab = Slab::new(((2, 0), 0), ((0, 0), 0));
        let pts: Vec<_> = slab.points().collect();
        assert_eq!(pts, vec![((0, 0), 0), ((1, 0), 0), ((2, 0), 0)]);
    }

    #[test]
    fn test_iter_y1() {
        let slab = Slab::new(((0, 0), 0), ((0, 2), 0));
        let pts: Vec<_> = slab.points().collect();
        assert_eq!(pts, vec![((0, 0), 0), ((0, 1), 0), ((0, 2), 0)]);
    }

    #[test]
    fn test_iter_y2() {
        let slab = Slab::new(((0, 2), 0), ((0, 0), 0));
        let pts: Vec<_> = slab.points().collect();
        assert_eq!(pts, vec![((0, 0), 0), ((0, 1), 0), ((0, 2), 0)]);
    }

    #[test]
    fn test_iter_z1() {
        let slab = Slab::new(((0, 0), 0), ((0, 0), 2));
        let pts: Vec<_> = slab.points().collect();
        assert_eq!(pts, vec![((0, 0), 0), ((0, 0), 1), ((0, 0), 2)]);
    }

    #[test]
    fn test_iter_z2() {
        let slab = Slab::new(((0, 0), 2), ((0, 0), 0));
        let pts: Vec<_> = slab.points().collect();
        assert_eq!(pts, vec![((0, 0), 0), ((0, 0), 1), ((0, 0), 2)]);
    }
}
