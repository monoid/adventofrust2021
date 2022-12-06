use std::{collections::BTreeMap, io::BufRead};

pub fn read_initial_state<R: BufRead>(inp: R) -> BTreeMap<u32, Vec<char>> {
    let mut crates: Vec<Vec<char>> = vec![];

    for init_line in inp
        .lines()
        .map(Result::unwrap)
        .map(|line| parse_initial_line(&line).unwrap().1)
    {
        match init_line {
            InitLine::Crates(line_vec) => {
                // We assume that line_vec.len is only growing.
                crates.resize(line_vec.len(), Default::default());
                for (r, s) in crates.iter_mut().zip(line_vec) {
                    if let Some(c) = s {
                        r.insert(0, c);
                    }
                }
            }
            InitLine::Labels(labels) => return labels.into_iter().zip(crates).collect(),
        }
    }
    panic!("Labels not found");
}

enum InitLine {
    Crates(Vec<Option<char>>),
    Labels(Vec<u32>),
}

fn parse_initial_line(inp: &str) -> nom::IResult<&str, InitLine> {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::u32 as parse_u32;
    use nom::combinator::{all_consuming, map};
    use nom::multi::separated_list1;
    use nom::sequence::delimited;

    all_consuming(alt((
        map(separated_list1(tag(" "), parse_init_cell), InitLine::Crates),
        map(
            separated_list1(tag(" "), delimited(tag(" "), parse_u32, tag(" "))),
            InitLine::Labels,
        ),
    )))(inp)
}

fn parse_init_cell(inp: &str) -> nom::IResult<&str, Option<char>> {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::anychar;
    use nom::combinator::{map, value};
    use nom::sequence::delimited;

    alt((
        value(None, tag("   ")),
        map(delimited(tag("["), anychar, tag("]")), Some),
    ))(inp)
}

#[derive(Debug)]
pub struct Move {
    pub count: usize,
    pub from: u32,
    pub to: u32,
}

pub fn read_moves<R: BufRead>(inp: R) -> Vec<Move> {
    inp.lines()
        .map(Result::unwrap)
        .filter(|flt| !flt.is_empty())
        .map(|line| parse_move(&line).unwrap().1)
        .collect()
}

fn parse_move(inp: &str) -> nom::IResult<&str, Move> {
    use nom::bytes::complete::tag;
    use nom::character::complete::u32 as parse_u32;
    use nom::combinator::map;
    use nom::sequence::{delimited, pair, separated_pair};

    map(
        pair(
            delimited(tag("move "), parse_u32, tag(" from ")),
            separated_pair(parse_u32, tag(" to "), parse_u32),
        ),
        |(count, (from, to))| Move {
            count: count as _,
            from,
            to,
        },
    )(inp)
}

pub fn apply_move1(ship: &mut BTreeMap<u32, Vec<char>>, mv: &Move) {
    for _ in 0..mv.count {
        let crate_ = ship.get_mut(&mv.from).unwrap().pop().unwrap();
        ship.get_mut(&mv.to).unwrap().push(crate_);
    }
}

pub fn apply_move2(ship: &mut BTreeMap<u32, Vec<char>>, mv: &Move) {
    let stack = ship.get_mut(&mv.from).unwrap();
    let len = stack.len();
    let crates: Vec<_> = stack.drain((len - mv.count)..).collect();
    ship.get_mut(&mv.to).unwrap().extend(crates.into_iter());
}
