use std::io;

use nom::{
    bytes::complete::{is_not, tag},
    combinator::{complete, map},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

pub struct Rule {
    pub name: String,
    range1: (u32, u32),
    range2: (u32, u32),
}

impl Rule {
    pub fn parse_rule(inp: &str) -> IResult<&str, Self> {
        map(
            complete(separated_pair(
                is_not(":"),
                tag(": "),
                separated_pair(parse_num_range, tag(" or "), parse_num_range),
            )),
            |(name, (range1, range2))| Self {
                name: name.to_owned(),
                range1,
                range2,
            },
        )(inp)
    }

    pub fn validate1(&self, item: u32) -> bool {
        (self.range1.0..=self.range1.1).contains(&item)
            || (self.range2.0..=self.range2.1).contains(&item)
    }
}

fn parse_num_range(inp: &str) -> IResult<&str, (u32, u32)> {
    use nom::character::complete::u32 as u32_parser;
    separated_pair(u32_parser, tag("-"), u32_parser)(inp)
}

pub fn parse_ticket(inp: &str) -> IResult<&str, Ticket> {
    use nom::character::complete::u32 as u32_parser;
    complete(separated_list1(tag(","), u32_parser))(inp)
}

pub type Ticket = Vec<u32>;

pub fn read_input<R: io::BufRead>(inp: R) -> (Vec<Rule>, Ticket, Vec<Ticket>) {
    let mut rules = vec![];
    let my_ticket;
    let mut tickets = vec![];

    let mut lines = inp.lines();
    loop {
        let line = lines.next().unwrap().unwrap();
        if line.is_empty() {
            break;
        } else {
            rules.push(Rule::parse_rule(&line).unwrap().1);
        }
    }

    lines.next(); // your ticket:
    my_ticket = parse_ticket(&lines.next().unwrap().unwrap()).unwrap().1;

    lines.next(); // empty
    lines.next(); // nearby tickets:

    for line in lines {
        tickets.push(parse_ticket(&line.unwrap()).unwrap().1);
    }

    (rules, my_ticket, tickets)
}

#[cfg(test)]
mod tests {
    use crate::Rule;

    #[test]
    fn test_parse_rule() {
        assert!(Rule::parse_rule("test me: 1-2 or 8-9").is_ok());
    }
}
