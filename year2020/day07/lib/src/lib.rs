use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{complete, map},
    sequence::{separated_pair, terminated},
};
use std::io;

#[derive(Debug, Clone, PartialEq)]
pub struct Rule {
    pub color: String,
    pub nested: Vec<(usize, String)>,
}

fn parse_color(v: &str) -> nom::IResult<&str, String> {
    use nom::character::complete::alpha1;
    map(separated_pair(alpha1, tag(" "), alpha1), |(a, b)| {
        format!("{} {}", a, b)
    })(v)
}

fn parse_content(v: &str) -> nom::IResult<&str, Vec<(usize, String)>> {
    terminated(
        alt((map(tag("no other bags"), |_| vec![]), parse_color_seq)),
        tag("."),
    )(v)
}

fn parse_color_seq(v: &str) -> nom::IResult<&str, Vec<(usize, String)>> {
    nom::multi::separated_list1(
        tag(", "),
        terminated(
            separated_pair(
                map(nom::character::complete::u32, |n| n as usize),
                tag(" "),
                parse_color,
            ),
            alt((tag(" bags"), tag(" bag"))),
        ),
    )(v)
}

pub fn parse_rule(v: &str) -> nom::IResult<&str, Rule> {
    complete(map(
        separated_pair(parse_color, tag(" bags contain "), parse_content),
        |(color, nested)| Rule { color, nested },
    ))(v)
}

pub fn read_rules<R: io::BufRead>(inp: R) -> Vec<Rule> {
    inp.lines()
        .map(|line| parse_rule(line.as_ref().unwrap()).unwrap().1)
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{parse_rule, Rule};
    #[test]
    fn test_simple() {
        let res = parse_rule("light red bags contain 1 bright white bag, 2 muted yellow bags.");
        let rule = res.unwrap().1;

        assert_eq!(
            rule,
            Rule {
                color: "light red".to_owned(),
                nested: vec![
                    (1, "bright white".to_owned()),
                    (2, "muted yellow".to_owned())
                ],
            }
        );
    }

    #[test]
    fn test_single() {
        let res = parse_rule("bright white bags contain 1 shiny gold bag.");
        let rule = res.unwrap().1;

        assert_eq!(
            rule,
            Rule {
                color: "bright white".to_owned(),
                nested: vec![(1, "shiny gold".to_owned()),],
            }
        );
    }

    #[test]
    fn test_empty() {
        let res = parse_rule("dotted black bags contain no other bags.");
        let rule = res.unwrap().1;

        assert_eq!(
            rule,
            Rule {
                color: "dotted black".to_owned(),
                nested: vec![],
            }
        );
    }
}
