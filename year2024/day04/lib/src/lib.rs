use std::io::BufRead;
use std::io::Read;

pub type Cell = u8;

pub struct Map {
    pub map: Vec<Vec<Cell>>,
}

pub type Pos = (usize, usize);

pub const TARGET: &[Cell] = b"XMAS";

impl Map {
    pub fn read<R: Read>(inp: R) -> Self {
        let buf_inp = std::io::BufReader::new(inp);
        let map = buf_inp.lines().map(|r| r.unwrap().into()).collect();
        Self { map }
    }

    pub fn height(&self) -> usize {
        self.map.len()
    }

    pub fn width(&self) -> usize {
        self.map[0].len()
    }

    pub fn around(&self, pos: Pos) -> impl Iterator<Item = ((isize, isize), Pos)> {
        use itertools::iproduct;
        let x = pos.0 as isize;
        let y = pos.1 as isize;
        let w = self.width() as isize;
        let h = self.height() as isize;

        iproduct!(-1isize..=1, -1isize..=1).filter_map(move |(dx, dy)| {
            let nx = x + dx;
            let ny = y + dy;
            if dx == 0 && dy == 0 || nx >= w || ny >= h {
                None // skip the origin
            } else {
                let coords = nx.try_into().ok().zip(ny.try_into().ok());
                Some((dx, dy)).zip(coords)
            }
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Match {
    pub left: usize,
    pub pos: Pos,
    pub dir: (isize, isize),
}

impl Match {
    pub fn advance<'map>(
        self,
        map: &'map Map,
        target: &[Cell],
    ) -> Option<impl Iterator<Item = Match> + 'map> {
        if self.left == 0 {
            None // complete match!
        } else {
            let target_val = target[target.len() - self.left];
            Some(map.around(self.pos).filter_map(move |(dir, p)| {
                if map.map[p.1][p.0] == target_val {
                    Some(Match {
                        left: self.left - 1,
                        pos: p,
                        dir,
                    })
                } else {
                    None
                }
            }))
        }
    }
}
