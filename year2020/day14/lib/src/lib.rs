use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{complete, map},
    sequence::{preceded, separated_pair},
    IResult,
};
use std::{collections::HashMap, io};

const WORD_SIZE: u32 = 36;

#[derive(Clone, Copy, Default)]
pub struct Mask {
    set: u64,
    unset: u64,
}

impl Mask {
    fn new(m: &str) -> Self {
        let mut set = 0u64;
        let mut unset = 0u64;

        assert!(m.len() == WORD_SIZE as usize);
        for (i, c) in m.bytes().rev().enumerate() {
            match c {
                b'0' => {
                    unset |= 1 << i;
                }
                b'1' => {
                    set |= 1 << i;
                }
                _ => {}
            }
        }

        Mask { set, unset }
    }

    fn apply(&self, val: u64) -> u64 {
        (val | self.set) & !self.unset
    }

    fn apply2(&self, val: u64) -> impl Iterator<Item = u64> {
        let unstable = !(self.set | self.unset) & ((1 << WORD_SIZE) - 1);
        let set = self.set;
        (0..(1 << unstable.count_ones())).map(move |comb| scatter(val, comb, set, unstable))
    }
}

fn scatter(val: u64, mut comb: u64, set: u64, unstable: u64) -> u64 {
    let mut result = 0;

    for i in 0..WORD_SIZE {
        let mask = 1 << i;
        if set & mask != 0 {
            // Nothing; do later
        } else if unstable & mask != 0 {
            result |= (comb & 1) << i;
            comb >>= 1;
        } else {
            result |= val & mask
        }
    }
    result | set
}

pub enum Cmd {
    Mask(Mask),
    Mem(u64, u64),
}

impl Cmd {
    pub fn parse(inp: &str) -> IResult<&str, Self> {
        complete(alt((
            map(
                preceded(tag("mask = "), nom::character::complete::alphanumeric0),
                |s: &str| Self::Mask(Mask::new(s)),
            ),
            map(
                preceded(
                    tag("mem["),
                    separated_pair(
                        nom::character::complete::u64,
                        tag("] = "),
                        nom::character::complete::u64,
                    ),
                ),
                |(a, v)| Self::Mem(a, v),
            ),
        )))(inp)
    }
}

pub fn read_input<R: io::BufRead>(inp: R) -> Vec<Cmd> {
    inp.lines()
        .map(|s| Cmd::parse(&s.unwrap()).unwrap().1)
        .collect()
}

pub fn execute(prog: &[Cmd]) -> u64 {
    let mut mem = HashMap::new();
    let mut mask = Mask::new("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"); // stub

    for cmd in prog {
        match cmd {
            Cmd::Mask(m) => mask = *m,
            &Cmd::Mem(addr, val) => {
                mem.insert(addr, mask.apply(val));
            }
        }
    }

    mem.values().sum()
}

pub fn execute2(prog: &[Cmd]) -> u64 {
    let mut mem = HashMap::new();
    let mut mask = Mask::new("000000000000000000000000000000000000"); // stub

    for cmd in prog {
        match cmd {
            Cmd::Mask(m) => mask = *m,
            &Cmd::Mem(addr, val) => {
                for real_addr in mask.apply2(addr) {
                    eprintln!("mem[{:36b}] = {}", real_addr, val);
                    mem.insert(real_addr, val);
                }
            }
        }
    }

    mem.values().sum()
}
