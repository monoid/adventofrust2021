use std::io;

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{complete, map, value},
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Op {
    Acc,
    Jmp,
    Nop,
}

impl Op {
    pub fn parse(v: &str) -> IResult<&str, Self> {
        alt((
            value(Op::Acc, tag("acc")),
            value(Op::Jmp, tag("jmp")),
            value(Op::Nop, tag("nop")),
        ))(v)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Cmd {
    pub op: Op,
    pub arg: isize,
}

impl Cmd {
    pub fn parse(v: &str) -> IResult<&str, Self> {
        complete(map(
            separated_pair(Op::parse, tag(" "), nom::character::complete::i32),
            |(op, arg)| Cmd {
                op,
                arg: arg as isize,
            },
        ))(v)
    }
}

pub fn read_prog<R: io::BufRead>(inp: R) -> Vec<Cmd> {
    inp.lines()
        .map(|line| Cmd::parse(&line.unwrap()).unwrap().1)
        .collect()
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Cpu {
    pub acc: isize,
    pub pc: usize,
}

impl Cpu {
    pub fn exec_cmd(&mut self, prog: &[Cmd]) -> bool {
        if self.pc >= prog.len() {
            return true;
        }

        let cmd = prog[self.pc];
        match cmd.op {
            Op::Acc => {
                self.acc += cmd.arg;
                self.pc += 1;
            }
            Op::Jmp => {
                self.pc = ((self.pc as isize) + cmd.arg) as usize;
            }
            Op::Nop => {
                self.pc += 1;
            }
        }
        self.pc >= prog.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Cmd, Op};

    #[test]
    fn test_nop() {
        let cmd = Cmd::parse("nop +0").unwrap().1;
        assert_eq!(
            cmd,
            Cmd {
                op: Op::Nop,
                arg: 0
            }
        )
    }

    #[test]
    fn test_acc() {
        let cmd = Cmd::parse("acc +1").unwrap().1;
        assert_eq!(
            cmd,
            Cmd {
                op: Op::Acc,
                arg: 1
            }
        )
    }

    #[test]
    fn test_jmp() {
        let cmd = Cmd::parse("jmp -4").unwrap().1;
        assert_eq!(
            cmd,
            Cmd {
                op: Op::Jmp,
                arg: -4
            }
        )
    }
}
