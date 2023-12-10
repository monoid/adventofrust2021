use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Tile {
    Grnd,
    Vert,
    Hor,
    NeL,
    NwJ,
    Sw7,
    SeF,
    // TOOD Starting point is just an 7 in our input data, but we'd like to distinguish it.
    Start,
}

pub type Pos = (usize, usize);

impl Tile {
    pub fn connections(self, p: Pos) -> Option<[Pos; 2]> {
        match self {
            Tile::Grnd => None,
            Tile::Vert => Some([(p.0, p.1 - 1), (p.0, p.1 + 1)]),
            Tile::Hor => Some([(p.0 - 1, p.1), (p.0 + 1, p.1)]),
            Tile::NeL => Some([(p.0 + 1, p.1), (p.0, p.1 - 1)]),
            Tile::NwJ => Some([(p.0 - 1, p.1), (p.0, p.1 - 1)]),
            Tile::Sw7 | Tile::Start => Some([(p.0 - 1, p.1), (p.0, p.1 + 1)]),
            Tile::SeF => Some([(p.0 + 1, p.1), (p.0, p.1 + 1)]),
        }
    }

    pub fn new_pos(self, char_pos: Pos, old_pos: Pos) -> Option<Pos> {
        self.connections(char_pos).map(|conn| {
            conn.iter()
                .copied()
                // The tile connects new and old pos; just remove old pos from the sequence,
                .filter(|n| n != &old_pos)
                // and make sure exactly one is left.
                .exactly_one()
                .expect("exactly one possible position expected")
        })
    }

    pub fn char(self) -> char {
        match self {
            Tile::Grnd => '.',
            Tile::Vert => '|',
            Tile::Hor => '-',
            Tile::NeL => 'L',
            Tile::NwJ => 'J',
            Tile::Sw7 => '7',
            Tile::SeF => 'F',
            Tile::Start => 'S',
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Tile::Grnd),
            '|' => Ok(Tile::Vert),
            '-' => Ok(Tile::Hor),
            'L' => Ok(Tile::NeL),
            'J' => Ok(Tile::NwJ),
            '7' => Ok(Tile::Sw7),
            'F' => Ok(Tile::SeF),
            'S' => Ok(Tile::Start),
            _ => Err("unknown title"),
        }
    }
}

pub fn read_data() -> Vec<Vec<Tile>> {
    std::io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            line.trim()
                .chars()
                .map(|c| Tile::try_from(c).unwrap())
                .collect_vec()
        })
        .collect_vec()
}

pub fn find_start_pos(data: &[Vec<Tile>]) -> Option<Pos> {
    data.iter().enumerate().find_map(|(y, line)| {
        line.iter().cloned().enumerate().find_map(|(x, t)| {
            if t == Tile::Start {
                Some((x, y))
            } else {
                None
            }
        })
    })
}

pub fn trace(start: Pos, map: &[Vec<Tile>]) -> Vec<Pos> {
    let mut trace = vec![start];
    let mut pos = start;
    let mut next = (start.0, start.1 + 1); // manual analysis

    while map[next.1][next.0] != Tile::Start {
        trace.push(next);
        let pos1 = map[next.1][next.0].new_pos(next, pos).unwrap();
        pos = next;
        next = pos1;
    }
    trace
}
