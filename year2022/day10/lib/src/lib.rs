#[derive(Debug, Copy, Clone)]
pub struct Cpu {
    x: isize,
    clock: usize,
}

impl Cpu {
    pub fn new() -> Self {
        Self { x: 1, clock: 0 }
    }

    pub fn execute(mut self, commands: &[Command]) -> impl Iterator<Item = (usize, isize)> + '_ {
        commands.iter().flat_map(move |cmd| {
            self.clock += 1;
            match cmd {
                Command::Noop => vec![(self.clock, self.x)],
                Command::Addx(d) => {
                    let prev_clock = self.clock;
                    let prev_x = self.x;
                    self.clock += 1;
                    self.x += d;
                    vec![(prev_clock, prev_x), (self.clock, prev_x)]
                }
            }
            .into_iter()
        })
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Command {
    Noop,
    Addx(isize),
}

pub fn load_data() -> Vec<Command> {
    use std::io::{self, BufRead};
    let stdin = io::stdin().lock();
    stdin
        .lines()
        .map(|r| parse_command(&r.unwrap()).unwrap().1)
        .collect()
}

fn parse_command(inp: &str) -> nom::IResult<&str, Command> {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::i32;
    use nom::combinator::{all_consuming, map, value};
    use nom::sequence::preceded;

    all_consuming(alt((
        value(Command::Noop, tag("noop")),
        map(preceded(tag("addx "), i32), |val| Command::Addx(val as _)),
    )))(inp)
}
