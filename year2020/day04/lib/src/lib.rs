use itertools::Itertools as _;
use std::{collections::HashMap, io};

fn check_byr(v: &str) -> bool {
    v.parse::<u32>()
        .map(|n| (1920..=2002).contains(&n))
        .unwrap_or(false)
}

fn check_iyr(v: &str) -> bool {
    v.parse::<u32>()
        .map(|n| (2010..=2020).contains(&n))
        .unwrap_or(false)
}

fn check_eyr(v: &str) -> bool {
    v.parse::<u32>()
        .map(|n| (2020..=2030).contains(&n))
        .unwrap_or(false)
}

fn check_hcl(v: &str) -> bool {
    use nom::bytes::complete::{tag, take_while_m_n};
    use nom::combinator::complete;
    use nom::sequence::pair;

    complete::<_, _, (), _>(pair(
        tag("#"),
        take_while_m_n(6, 6, |c: char| c.is_ascii_hexdigit()),
    ))(v)
    .is_ok()
}

fn check_height(v: &str) -> bool {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::u32;
    use nom::combinator::{complete, map};
    use nom::sequence::pair;

    complete::<_, _, (), _>(map(pair(u32, alt((tag("cm"), tag("in")))), |(h, u)| {
        if u == "cm" {
            (150..=193).contains(&h)
        } else {
            (59..=76).contains(&h)
        }
    }))(v)
    .map(|(_, r)| r)
    .unwrap_or(false)
}

fn check_ecl(v: &str) -> bool {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::combinator::complete;
    complete::<_, _, (), _>(alt((
        tag("amb"),
        tag("blu"),
        tag("brn"),
        tag("gry"),
        tag("grn"),
        tag("hzl"),
        tag("oth"),
    )))(v)
    .is_ok()
}

fn check_pid(v: &str) -> bool {
    v.len() == 9 && v.chars().all(|c| c.is_ascii_digit())
}

pub const REQUIRED: [(&str, fn(&str) -> bool); 7] = [
    ("byr", check_byr),
    ("iyr", check_iyr),
    ("eyr", check_eyr),
    ("hgt", check_height),
    ("hcl", check_hcl),
    ("ecl", check_ecl),
    ("pid", check_pid),
    // ("cid", stub),
];

fn records<R: io::BufRead>(input: R) -> impl Iterator<Item = String> {
    let data = input.lines().map(Result::unwrap).collect_vec();

    data.into_iter().batching(|it| {
        let mut buf = String::new();
        loop {
            match it.next() {
                None => {
                    if buf.is_empty() {
                        return None;
                    } else {
                        return Some(buf);
                    }
                }
                Some(other) => {
                    if other.is_empty() {
                        return Some(buf);
                    } else {
                        if !buf.is_empty() {
                            buf.push(' ');
                        }
                        buf.push_str(&other);
                    }
                }
            }
        }
    })
}

pub fn read_data() -> Vec<HashMap<String, String>> {
    let stdin = io::stdin();
    records(stdin.lock())
        .map(|group| {
            group
                .split_ascii_whitespace()
                .map(|item| {
                    let (a, b) = item.split_once(':').unwrap();
                    (a.to_owned(), b.to_owned())
                })
                .collect()
        })
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use std::io;

    use itertools::Itertools;

    use crate::records;

    #[test]
    fn test_name() {
        let data = records(io::BufReader::new(
            "one two\nthree\n\nfour\nfive\n".as_bytes(),
        ))
        .collect_vec();
        assert_eq!(
            data,
            vec!["one two three".to_owned(), "four five".to_owned(),]
        )
    }
}
