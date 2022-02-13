use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{complete::anychar, streaming::char},
    combinator::{complete, map},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult,
};
use std::{collections::HashMap, io};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Rule {
    Char(u8),
    Rules(Vec<Vec<u8>>),
    Nd(u8, u8),
}

impl Rule {
    pub fn parse(inp: &str) -> IResult<&str, (u8, Rule)> {
        use nom::character::complete::u8 as u8_parse;
        separated_pair(u8_parse, tag(": "), parse_right_side)(inp)
    }

    fn match_line<'a>(&self, rules: &HashMap<u8, Rule>, data: &'a [u8]) -> Option<&'a [u8]> {
        match self {
            &Rule::Char(b) => {
                data.split_first().and_then(
                    |(&first, rest)| {
                        if first == b {
                            Some(rest)
                        } else {
                            None
                        }
                    },
                )
            }
            Rule::Rules(alt) => alt
                .iter()
                .map(|seq| {
                    seq.iter().try_fold(data, |dat, rule_id| {
                        rules.get(rule_id).unwrap().match_line(rules, dat)
                    })
                })
                .find_map(|x| x),
            &Rule::Nd(ra, rb) => {
                if (0..=(data.len())).any(|mid| {
                    let (da, db) = data.split_at(mid);
                    matches!(rules.get(&ra).unwrap().match_line(rules, da), Some(&[]))
                        && matches!(rules.get(&rb).unwrap().match_line(rules, db), Some(&[]))
                }) {
                    Some(&[])
                } else {
                    None
                }
            }
        }
    }
}

fn parse_right_side(inp: &str) -> IResult<&str, Rule> {
    use nom::character::complete::u8 as u8_parse;
    alt((
        map(delimited(char('"'), anychar, char('"')), |c| {
            Rule::Char(c as _)
        }),
        map(
            complete(separated_list0(
                tag(" | "),
                complete(separated_list1(tag(" "), u8_parse)),
            )),
            Rule::Rules,
        ),
    ))(inp)
}

pub fn read_rules<R: io::BufRead>(inp: R) -> Result<HashMap<u8, Rule>, io::Error> {
    inp.lines()
        .take_while(|lr| lr.as_ref().map(|l| !l.is_empty()).unwrap_or_default())
        .map(|lr| lr.map(|l| Rule::parse(&l).unwrap().1))
        .collect()
}

pub fn match_string(line: &[u8], rules: &HashMap<u8, Rule>) -> bool {
    rules
        .get(&0)
        .unwrap()
        .match_line(rules, line)
        .map(|x| x.is_empty())
        .unwrap_or(false)
}

pub fn count_matches<R: io::BufRead>(inp: R, rules: &HashMap<u8, Rule>) -> usize {
    inp.lines()
        .filter(|lr| match_string(lr.as_ref().unwrap().as_bytes(), rules))
        .count()
}

#[cfg(test)]
mod tests {
    use crate::Rule;

    #[test]
    fn test_parse_rule_char() {
        let res = Rule::parse("0: \"a\"").unwrap().1;
        assert_eq!(res, (0, Rule::Char(b'a')));
    }

    #[test]
    fn test_parse_rule_alt() {
        let res = Rule::parse("0: 2 3 | 3 2").unwrap().1;
        assert_eq!(res, (0, Rule::Rules(vec![vec![2, 3], vec![3, 2]])));
    }

    #[test]
    fn test_parse_rule_single() {
        let res = Rule::parse("0: 2 3 2").unwrap().1;
        assert_eq!(res, (0, Rule::Rules(vec![vec![2, 3, 2]])));
    }
}
