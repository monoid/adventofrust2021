use either::{Left, Right};
use itertools::Itertools as _;
use nom::{bytes::complete::tag, combinator::complete, sequence::delimited, IResult};
use std::{
    collections::{HashMap, HashSet},
    io,
};

fn usqrt(v: usize) -> usize {
    (v as f64).sqrt() as _
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Side {
    Top,
    TopRev,
    Left,
    LeftRev,
    Bottom,
    BottomRev,
    Right,
    RightRev,
}

#[derive(Clone, Copy, Debug)]
pub enum Edge {
    Top,
    Left,
}

impl Default for Side {
    fn default() -> Self {
        Side::Top
    }
}

#[derive(Default, Clone)]
pub struct Tile(u16, Vec<Vec<u8>>, Side);

fn reverse<T>(mut data: Vec<T>) -> Vec<T> {
    data.reverse();
    data
}

impl Tile {
    pub fn id(&self) -> u16 {
        self.0
    }

    pub fn left(&self) -> Vec<u8> {
        self.1.iter().map(|l| l[0]).collect()
    }

    pub fn right(&self) -> Vec<u8> {
        self.1.iter().map(|l| l[l.len() - 1]).collect()
    }

    pub fn top(&self) -> &[u8] {
        &self.1[0]
    }

    pub fn bottom(&self) -> &[u8] {
        self.1.last().unwrap()
    }

    #[inline]
    pub fn edge(&self, side: Side) -> Vec<u8> {
        match side {
            Side::Top => self.1[0].clone(),
            Side::TopRev => reverse(self.1[0].clone()),
            Side::Left => self.left(),
            Side::LeftRev => reverse(self.left()),
            Side::Bottom => self.1.last().unwrap().clone(),
            Side::BottomRev => reverse(self.1.last().unwrap().clone()),
            Side::Right => self.right(),
            Side::RightRev => reverse(self.right()),
        }
    }

    pub fn edges(&self) -> Vec<(Side, Vec<u8>)> {
        [
            Side::Top,
            Side::TopRev,
            Side::Left,
            Side::LeftRev,
            Side::Bottom,
            Side::BottomRev,
            Side::Right,
            Side::RightRev,
        ]
        .into_iter()
        .map(|side| (side, self.edge(side)))
        .collect()
    }

    fn rotate(&self, lrev: bool, rrev: bool) -> Self {
        let mut data = vec![vec![]; self.1[0].len()];
        for l in if lrev {
            Left(self.1.iter().rev())
        } else {
            Right(self.1.iter())
        } {
            for (out, inp) in data.iter_mut().zip(if rrev {
                Left(l.iter().rev().cloned())
            } else {
                Right(l.iter().cloned())
            }) {
                out.push(inp);
            }
        }
        Self(self.0, data, Default::default())
    }

    pub fn flip(&self, lrev: bool, rrev: bool) -> Self {
        Self(
            self.0,
            (if lrev {
                Left(self.1.iter().rev())
            } else {
                Right(self.1.iter())
            })
            .map(|l| if rrev { reverse(l.clone()) } else { l.clone() })
            .collect(),
            Default::default(),
        )
    }

    pub fn transform_to(&self, matched_side: Side, to: Edge) -> Self {
        match (matched_side, to) {
            (Side::Top, Edge::Top) | (Side::Left, Edge::Left) => self.clone(),
            (Side::TopRev, Edge::Top) | (Side::Right, Edge::Left) => self.flip(false, true),
            (Side::LeftRev, Edge::Left) | (Side::Bottom, Edge::Top) => self.flip(true, false),
            (Side::BottomRev, Edge::Top) | (Side::RightRev, Edge::Left) => self.flip(true, true),
            (Side::Top, Edge::Left) | (Side::Left, Edge::Top) => self.rotate(false, false),
            (Side::Right, Edge::Top) | (Side::TopRev, Edge::Left) => self.rotate(false, true),
            (Side::LeftRev, Edge::Top) | (Side::Bottom, Edge::Left) => self.rotate(true, false),
            (Side::BottomRev, Edge::Left) | (Side::RightRev, Edge::Top) => self.rotate(true, true),
        }
    }

    pub fn subtile(&'_ self) -> impl Iterator<Item = &'_ [u8]> + '_ {
        let h = self.1.len();
        let w = self.1[0].len();
        self.1[1..(h - 1)].iter().map(move |line| &line[1..(w - 1)])
    }
}

fn parse_header(inp: &str) -> IResult<&str, u16> {
    use nom::character::complete::u16 as u16_parse;
    complete(delimited(tag("Tile "), u16_parse, tag(":")))(inp)
}

pub fn read_tiles<R: io::BufRead>(inp: R) -> Vec<Tile> {
    inp.lines()
        .batching(|it| {
            if let Some(header) = it.next() {
                let id = parse_header(&header.unwrap()).unwrap().1;
                let mut tile = Tile(id, Default::default(), Default::default());
                for line in it.by_ref() {
                    let line = line.unwrap();
                    if line.is_empty() {
                        break;
                    } else {
                        tile.1.push(line.into());
                    }
                }

                if tile.1.is_empty() {
                    None
                } else {
                    Some(tile)
                }
            } else {
                None
            }
        })
        .collect()
}

pub fn compose_tiles(data: &[Tile]) -> Vec<Vec<Tile>> {
    let mut data_by_id: HashMap<u16, &Tile> =
        HashMap::from_iter(data.iter().map(|tile| (tile.id(), tile)));

    let mut hm = HashMap::<Vec<u8>, HashSet<(Side, u16)>>::default();

    for tile in data {
        for (side, edge) in tile.edges() {
            hm.entry(edge).or_default().insert((side, tile.id()));
        }
    }

    let mut tile_counts = HashMap::<_, HashSet<Side>>::default();

    // There are 96 1-element ids and 528 2-element ids; no false duplicates.
    for ids in hm.values() {
        if ids.len() == 2 {
            for (side, id) in ids {
                tile_counts.entry(id).or_default().insert(*side);
            }
        }
    }

    // Corner tiles have exactly 2 unmatched sides.  But we
    // store each side twice: direct and reversed.  So, 4 in total.
    // Theoretically, there can be false matches, if some tile has
    // same side and other, but it conflicts with other sides,
    // but it is not true for my input data.
    //
    // So, this is a tile that has 4 non-matching sides.
    let (&start_id, start_items) = tile_counts
        .into_iter()
        .find(|(_, cnt)| cnt.len() == 4)
        .expect("Cannot find a corner tile without brute force search");
    let start_side: Side = *start_items.iter().next().unwrap();
    let field_height = usqrt(data.len());
    let field_width = field_height;
    assert_eq!(data.len(), field_height * field_width);
    let mut output: Vec<Vec<Tile>> = vec![Vec::with_capacity(field_width); field_height];
    let mut initial_tile = data_by_id
        .remove(&start_id)
        .unwrap()
        .transform_to(start_side, Edge::Top);
    if hm.get(&initial_tile.left()).unwrap().len() > hm.get(&initial_tile.right()).unwrap().len() {
        initial_tile = initial_tile.flip(false, true);
    }
    if hm.get(initial_tile.top()).unwrap().len() > hm.get(initial_tile.bottom()).unwrap().len() {
        initial_tile = initial_tile.flip(true, false);
    }
    output[0].push(initial_tile);

    for i in 1..field_width {
        let prev_edge = output[0][i - 1].right();
        let hm_items = hm.get(&prev_edge).unwrap();
        let (next_side, next_tile) = hm_items
            .iter()
            .find_map(|(side, id)| data_by_id.remove(id).map(|tile| (*side, tile)))
            .expect("Failed to find next tile");
        let new_tile = next_tile.transform_to(next_side, Edge::Left);
        assert_eq!(prev_edge, new_tile.left());
        output[0].push(new_tile);
    }

    for line in 1..field_height {
        for i in 0..field_width {
            let prev_edge = output[line - 1][i].bottom();
            let hm_items = hm.get(prev_edge).unwrap();
            let (next_side, next_tile) = hm_items
                .iter()
                .find_map(|(side, id)| data_by_id.remove(id).map(|tile| (*side, tile)))
                .expect("Failed to find next tile");
            let new_tile = next_tile.transform_to(next_side, Edge::Top);
            assert_eq!(prev_edge, new_tile.top());
            output[line].push(new_tile);
        }
    }
    output
}

pub fn merge(composed: &[Vec<Tile>]) -> Vec<Vec<u8>> {
    composed
        .iter()
        .flat_map(|tile_row| {
            let mut rows = vec![vec![]; tile_row[0].1.len() - 2];
            let mut tile_its: Vec<_> = tile_row.iter().map(|tile| tile.subtile()).collect();
            for cell in &mut rows {
                *cell = tile_its
                    .iter_mut()
                    .flat_map(|ch| ch.next().unwrap())
                    .cloned()
                    .collect();
            }
            assert!(tile_its.iter_mut().all(|it| it.next().is_none()));
            rows
        })
        .collect()
}

const PATTERN: &str = "                  #
#    ##    ##    ###
 #  #  #  #  #  #   ";

pub fn get_pattern_indices() -> Vec<(usize, usize)> {
    PATTERN
        .lines()
        .enumerate()
        .flat_map(|(lidx, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(pidx, c)| if c == '#' { Some((pidx, lidx)) } else { None })
        })
        .collect()
}

pub fn patterns() -> Vec<Vec<(usize, usize)>> {
    let patt = get_pattern_indices();
    let sy = 3;
    let sx = 20;

    (0..8)
        .map(|idx| {
            let flip = (idx & 1) != 0;
            let xsign = (idx & 2) == 0;
            let ysign = (idx & 4) == 0;
            patt.iter()
                .map(|&(mut x, mut y)| {
                    if xsign {
                        x = sx - x;
                    }
                    if ysign {
                        y = sy - y;
                    }
                    if flip {
                        std::mem::swap(&mut x, &mut y);
                    }
                    (x, y)
                })
                .collect()
        })
        .collect()
}
