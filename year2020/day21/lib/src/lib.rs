use std::{
    collections::{HashMap, HashSet},
    io,
};

use nom::{
    bytes::complete::tag,
    combinator::{complete, map},
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    IResult,
};

pub type Line = (HashSet<String>, HashSet<String>);
pub fn parse_line(inp: &str) -> IResult<&str, Line> {
    use nom::character::complete::alpha1;
    map(
        complete(separated_pair(
            separated_list1(tag(" "), map(alpha1, str::to_owned)),
            tag(" (contains "),
            terminated(
                separated_list1(tag(", "), map(alpha1, str::to_owned)),
                tag(")"),
            ),
        )),
        |(a, b)| {
            (
                HashSet::from_iter(a.into_iter()),
                HashSet::from_iter(b.into_iter()),
            )
        },
    )(inp)
}

pub fn read_data<R: io::BufRead>(inp: R) -> Vec<Line> {
    inp.lines()
        .map(|line| parse_line(&line.unwrap()).unwrap().1)
        .collect()
}

pub fn calc_product_dict(data: &[Line]) -> HashMap<String, String> {
    let mut prod_hash: HashMap<String, HashSet<String>> = HashMap::default();

    for it in data {
        for prod in &it.1 {
            if let Some(prev) = prod_hash.get_mut(prod) {
                *prev = prev.intersection(&it.0).cloned().collect()
            } else {
                prod_hash.insert(prod.clone(), it.0.clone());
            }
        }
    }

    let mut diag: Vec<_> = prod_hash.into_iter().collect();
    for idx in 0..diag.len() {
        diag[idx..].sort_unstable_by_key(|v| v.1.len());
        assert_eq!(diag[idx].1.len(), 1);
        let m = diag[idx].1.iter().next().unwrap().clone();
        for nxt in &mut diag[(idx + 1)..] {
            nxt.1.remove(&m);
        }
    }
    HashMap::from_iter(
        diag.into_iter()
            .map(|(k, v)| (k, v.into_iter().next().unwrap())),
    )
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::parse_line;

    #[test]
    fn test_parse() {
        let p = parse_line("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)").unwrap();
        assert_eq!(
            p.1,
            (
                HashSet::from_iter(
                    [
                        "mxmxvkd".to_string(),
                        "kfcds".to_string(),
                        "sqjhc".to_string(),
                        "nhms".to_string(),
                    ]
                    .into_iter()
                ),
                HashSet::from_iter(["dairy".to_string(), "fish".to_string()].into_iter()),
            )
        );
    }

    #[test]
    fn test_parse_single() {
        let p = parse_line("mxmxvkd kfcds sqjhc nhms (contains dairy)").unwrap();
        assert_eq!(
            p.1,
            (
                HashSet::from_iter(
                    [
                        "mxmxvkd".to_string(),
                        "kfcds".to_string(),
                        "sqjhc".to_string(),
                        "nhms".to_string(),
                    ]
                    .into_iter()
                ),
                HashSet::from_iter(["dairy".to_string()].into_iter()),
            )
        );
    }
}
