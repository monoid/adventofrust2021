use std::{
    fmt::{self, Display},
    io,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Floor,
    Empty,
    Occupied,
}

impl Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                State::Floor => ".",
                State::Empty => "L",
                State::Occupied => "#",
            }
        )
    }
}

impl TryFrom<char> for State {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(State::Floor),
            'L' => Ok(State::Empty),
            '#' => Ok(State::Occupied),
            _ => Err("unknown char found"),
        }
    }
}

pub type Map = Vec<Vec<State>>;

pub fn read_map<R: io::BufRead>(inp: R) -> Map {
    inp.lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(State::try_from)
                .collect::<Result<Vec<_>, _>>()
                .unwrap()
        })
        .collect()
}
