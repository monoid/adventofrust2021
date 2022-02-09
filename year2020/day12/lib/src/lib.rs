use std::io;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Cmd {
    N(i32),
    S(i32),
    E(i32),
    W(i32),
    L(i32),
    F(i32),
}

impl Cmd {
    pub fn parse(inp: &str) -> nom::IResult<&str, Cmd> {
        nom::combinator::map(
            nom::sequence::pair(
                nom::character::complete::anychar,
                nom::character::complete::i32,
            ),
            |(c, val)| match c {
                'N' => Cmd::N(val),
                'S' => Cmd::S(val),
                'E' => Cmd::E(val),
                'W' => Cmd::W(val),
                'L' => Cmd::L(val),
                'R' => Cmd::L(360 - val),
                'F' => Cmd::F(val),
                _ => panic!("Unknown command {:?}", c),
            },
        )(inp)
    }
}

pub fn read_commands<R: io::BufRead>(inp: R) -> impl Iterator<Item = Cmd> {
    inp.lines()
        .map(|line| Cmd::parse(&line.unwrap()).unwrap().1)
}

pub struct Ship {
    pub dir: i32,
    pub x: i32,
    pub y: i32,

    pub wx: i32,
    pub wy: i32,
}

impl Ship {
    pub fn new() -> Self {
        Self {
            dir: 0,
            x: 0,
            y: 0,
            wx: 10,
            wy: 1,
        }
    }

    pub fn apply(&mut self, cmd: Cmd) {
        match cmd {
            Cmd::N(n) => self.y += n,
            Cmd::S(s) => self.y -= s,
            Cmd::E(e) => self.x += e,
            Cmd::W(w) => self.x -= w,
            Cmd::L(ang) => self.dir = (self.dir + ang) % 360,
            Cmd::F(dist) => match self.dir {
                0 => self.x += dist,
                90 => self.y += dist,
                180 => self.x -= dist,
                270 => self.y -= dist,
                _ => panic!("Cannot handle direction {}", self.dir),
            },
        }
    }

    pub fn apply_wp(&mut self, cmd: Cmd) {
        match cmd {
            Cmd::N(n) => self.wy += n,
            Cmd::S(s) => self.wy -= s,
            Cmd::E(e) => self.wx += e,
            Cmd::W(w) => self.wx -= w,
            Cmd::L(ang) => match ang {
                0 => {}
                90 => {
                    let a = (self.wx, self.wy);
                    self.wx = -a.1;
                    self.wy = a.0;
                }
                180 => {
                    let a = (self.wx, self.wy);
                    self.wx = -a.0;
                    self.wy = -a.1;
                }
                270 => {
                    let a = (self.wx, self.wy);
                    self.wx = a.1;
                    self.wy = -a.0;
                }
                _ => panic!("Cannot handle direction {}", ang),
            },
            Cmd::F(dist) => {
                self.x += dist * self.wx;
                self.y += dist * self.wy;
            }
        }
    }

    pub fn distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}
