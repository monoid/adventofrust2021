use search::find_longest_path;

mod search;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Cell {
    Wall = b'#',
    Floor = b'.',
    R = b'>',
    L = b'<',
    U = b'^',
    D = b'v',
}

impl From<u8> for Cell {
    fn from(value: u8) -> Self {
        match value {
            b'#' => Cell::Wall,
            b'.' => Cell::Floor,
            b'>' => Cell::R,
            b'<' => Cell::L,
            b'^' => Cell::U,
            b'v' => Cell::D,
            _ => panic!("unknown char: {}", value),
        }
    }
}

type Pos = (usize, usize);

pub struct Map {
    pub data: Vec<Vec<Cell>>,
    pub start: Pos,
    pub end: Pos,
    pub slippery: bool,
}

impl Map {
    pub fn read_data() -> Self {
        let data: Vec<Vec<Cell>> = std::io::stdin()
            .lines()
            .map(|line| {
                line.unwrap()
                    .as_bytes()
                    .iter()
                    .cloned()
                    .map(Into::into)
                    .collect()
            })
            .collect();
        let start_x = data[0]
            .iter()
            .position(|&r| r == Cell::Floor)
            .expect("No start found");
        let end_x = data
            .last()
            .unwrap()
            .iter()
            .position(|&r| r == Cell::Floor)
            .expect("No end found");
        let h = data.len();
        Self {
            data,
            start: (start_x, 0),
            end: (end_x, h - 1),
            slippery: true,
        }
    }

    #[inline]
    pub fn find_longest_path_len(&self) -> usize {
        // exclude starting point
        find_longest_path(self) - 1
    }

    #[inline]
    pub(crate) fn around(&self, pos: Pos) -> impl Iterator<Item = Pos> + '_ {
        let ix = pos.0 as isize;
        let iy = pos.1 as isize;
        let w = self.data[0].len();
        let h = self.data.len();

        [(-1isize, 0isize), (1, 0), (0, 1), (0, -1)]
            .into_iter()
            .filter_map(move |(dx, dy)| {
                let nx = ix + dx;
                let ny = iy + dy;

                let npos = nx
                    .try_into()
                    .and_then(|unx: usize| ny.try_into().map(move |uny: usize| (unx, uny)))
                    .ok();

                npos.filter(|(nx, ny)| nx < &w && ny < &h && self.valid_move(pos, (*nx, *ny)))
            })
    }

    #[inline]
    fn valid_move(&self, frm: Pos, to: Pos) -> bool {
        if self.data[to.1][to.0] == Cell::Wall {
            return false;
        }
        if self.slippery {
            match self.data[frm.1][frm.0] {
                Cell::Wall => unreachable!(),
                Cell::Floor => true,
                Cell::R => (frm.0 + 1, frm.1) == to,
                Cell::L => (to.0 + 1, to.1) == frm,
                Cell::U => (to.0, to.1 + 1) == frm,
                Cell::D => (frm.0, frm.1 + 1) == to,
            }
        } else {
            match self.data[frm.1][frm.0] {
                Cell::Wall => unreachable!(),
                _ => true,
            }
        }
    }
}

impl std::fmt::Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let map_debug = MapDebug(
            self.data
                .iter()
                .map(|l| {
                    l.iter()
                        .copied()
                        .map(|n| (n as u8) as char)
                        .collect::<String>()
                })
                .collect(),
        );

        f.debug_struct("Map")
            .field("start", &self.start)
            .field("end", &self.end)
            .field("data", &map_debug)
            .finish()
    }
}

struct MapDebug(Vec<String>);

impl std::fmt::Debug for MapDebug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.0.iter()).finish()
    }
}
