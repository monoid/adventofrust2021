#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum MapCell {
    Empty = b'.',
    MirrorSlash = b'/',
    MirrorBackslash = b'\\',
    SplitterV = b'|',
    SplitterH = b'-',
}

impl MapCell {
    pub fn transform(self, dir: Dir) -> (Dir, Option<Dir>) {
        match self {
            MapCell::Empty => (dir, None),
            MapCell::MirrorSlash => transform_mirror_slash(dir),
            MapCell::MirrorBackslash => transform_mirror_backslash(dir),
            MapCell::SplitterV => transform_splitter_v(dir),
            MapCell::SplitterH => transform_splitter_h(dir),
        }
    }
}

impl TryFrom<u8> for MapCell {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            b'.' => MapCell::Empty,
            b'/' => MapCell::MirrorSlash,
            b'\\' => MapCell::MirrorBackslash,
            b'|' => MapCell::SplitterV,
            b'-' => MapCell::SplitterH,
            _ => return Err(()),
        })
    }
}

// The /
fn transform_mirror_slash(dir: Dir) -> (Dir, Option<Dir>) {
    let new_dir = match dir {
        Dir::Right => Dir::Up,
        Dir::Left => Dir::Down,
        Dir::Up => Dir::Right,
        Dir::Down => Dir::Left,
    };

    (new_dir, None)
}

// The |
fn transform_splitter_v(dir: Dir) -> (Dir, Option<Dir>) {
    match dir {
        Dir::Right | Dir::Left => (Dir::Up, Some(Dir::Down)),
        Dir::Up | Dir::Down => (dir, None),
    }
}

// The -
fn transform_splitter_h(dir: Dir) -> (Dir, Option<Dir>) {
    match dir {
        Dir::Right | Dir::Left => (dir, None),
        Dir::Up | Dir::Down => (Dir::Left, Some(Dir::Right)),
    }
}

// The \
fn transform_mirror_backslash(dir: Dir) -> (Dir, Option<Dir>) {
    let new_dir = match dir {
        Dir::Right => Dir::Down,
        Dir::Left => Dir::Up,
        Dir::Up => Dir::Left,
        Dir::Down => Dir::Right,
    };

    (new_dir, None)
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Dir {
    Right = 0,
    Left = 1,
    Up = 2,
    Down = 3,
}

impl Dir {
    pub fn step(self, pos: Pos) -> Pos {
        match self {
            Dir::Right => (pos.0 + 1, pos.1),
            Dir::Left => (pos.0 - 1, pos.1),
            Dir::Up => (pos.0, pos.1 - 1),
            Dir::Down => (pos.0, pos.1 + 1),
        }
    }
}
// *OUTGOING* beams
#[derive(Debug, Copy, Clone, Default)]
pub struct State([bool; 4]);

impl State {
    pub fn mark(&mut self, dir: Dir) -> bool {
        let dir_code = dir as usize;
        let old_val = self.0[dir_code];
        self.0[dir_code] = true;
        old_val
    }

    pub fn any(&self) -> bool {
        self.0.iter().any(|x| *x)
    }
}

pub fn read_cells() -> Vec<Vec<MapCell>> {
    std::io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let line = line.trim().as_bytes();
            line.iter()
                .cloned()
                .map(|b| MapCell::try_from(b).unwrap())
                .collect()
        })
        .collect()
}

pub type Pos = (isize, isize);

#[derive(Debug, Clone)]
pub struct Map {
    pub cells: Vec<Vec<MapCell>>,
    pub state: Vec<Vec<State>>,
}

impl Map {
    pub fn get(&mut self, pos: Pos) -> Option<(MapCell, &mut State)> {
        let x: usize = pos.0.try_into().ok()?;
        let y: usize = pos.1.try_into().ok()?;

        Some((
            *self.cells.get(y)?.get(x)?,
            self.state.get_mut(y)?.get_mut(x)?,
        ))
    }
}

pub fn read_data() -> Map {
    let cells = read_cells();
    let state = vec![vec![State::default(); cells[0].len()]; cells.len()];

    Map { cells, state }
}

pub fn find_energy(init_pos: (Pos, Dir), mut map: Map) -> usize {
    let mut stack = vec![init_pos];

    while let Some(mut beam) = stack.pop() {
        // While the beam is within boundaries.
        while let Some((cell, state)) = map.get(beam.0) {
            let (new_dir, secondary) = cell.transform(beam.1);
            if let Some(sec_dir) = secondary {
                state.mark(sec_dir);
                stack.push((sec_dir.step(beam.0), sec_dir));
            }
            let old_state = state.mark(new_dir);
            if old_state {
                break; // We have already passed to that direction, nothing new.
            }
            beam = (new_dir.step(beam.0), new_dir);
        }
    }

    let count = map
        .state
        .iter()
        .map(|line| line.iter().filter(|s| s.any()).count())
        .sum();
    count
}
