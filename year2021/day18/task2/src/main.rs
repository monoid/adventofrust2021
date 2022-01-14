use itertools::Itertools as _;
use std::{
    fmt::Debug,
    io::{self, BufRead as _},
    str::FromStr,
};

use nom::{
    branch::alt,
    character::complete::{char, digit1},
    combinator, sequence,
};

#[derive(Debug, Clone)]
enum Val<N: 'static> {
    Single(N),
    Pair(Box<(Val<N>, Val<N>)>),
}

impl Val<u32> {
    fn magn(&self) -> u32 {
        match self {
            Val::Single(v) => *v,
            Val::Pair(p) => 3 * p.0.magn() + 2 * p.1.magn(),
        }
    }
}

fn parse_pair<N: FromStr>(s: &str) -> nom::IResult<&str, Val<N>>
where
    <N as FromStr>::Err: Debug,
{
    combinator::map(
        sequence::delimited(
            char('['),
            sequence::separated_pair(parse_tree, char(','), parse_tree),
            char(']'),
        ),
        |p| Val::Pair(Box::new(p)),
    )(s)
}

fn parse_num<N: FromStr>(s: &str) -> nom::IResult<&str, Val<N>>
where
    <N as FromStr>::Err: Debug,
{
    combinator::map_res(digit1, |t: &str| t.parse().map(Val::Single))(s)
}

fn parse_tree<N: FromStr>(s: &str) -> nom::IResult<&str, Val<N>>
where
    <N as FromStr>::Err: Debug,
{
    alt((parse_num, parse_pair))(s)
}

fn add_val(v1: Val<u32>, v2: Val<u32>) -> Val<u32> {
    reduce_num(Val::Pair(Box::new((v1, v2))))
}

type Res<T> = Result<T, T>;

fn try_reduce_num(v1: &mut Val<u32>) -> Res<()> {
    explode(v1)?;
    split(v1)
}

fn reduce_num(mut v1: Val<u32>) -> Val<u32> {
    loop {
        match try_reduce_num(&mut v1) {
            Ok(_) => return v1,
            Err(_) => {
                continue;
            }
        }
    }
}

fn explode(v1: &mut Val<u32>) -> Res<()> {
    explode_find(v1, None, None, 0)
}

fn explode_find(
    v: &mut Val<u32>,
    left: Option<&mut Val<u32>>,
    right: Option<&mut Val<u32>>,
    level: usize,
) -> Res<()> {
    if level < 4 {
        match v {
            Val::Single(_) => Ok(()),
            Val::Pair(p) => {
                explode_find(&mut p.0, left, Some(&mut p.1), level + 1)?;
                explode_find(&mut p.1, Some(&mut p.0), right, level + 1)
            }
        }
    } else {
        match v {
            Val::Single(_) => Ok(()),
            Val::Pair(p) => {
                if let (Val::Single(a1), Val::Single(a2)) = p.as_ref() {
                    if let Some(l) = left.map(|n| most(n, false)) {
                        *l += a1;
                    }
                    if let Some(r) = right.map(|n| most(n, true)) {
                        *r += a2;
                    }
                    *v = Val::Single(0);
                    Err(())
                } else {
                    explode_find(&mut p.0, left, Some(&mut p.1), level + 1)?;
                    explode_find(&mut p.1, Some(&mut p.0), right, level + 1)
                }
            }
        }
    }
}

fn most<N>(v: &mut Val<N>, left: bool) -> &mut N {
    match v {
        Val::Single(n) => n,
        Val::Pair(p) => {
            if left {
                most(&mut p.0, left)
            } else {
                most(&mut p.1, left)
            }
        }
    }
}

fn split(v1: &mut Val<u32>) -> Res<()> {
    match v1 {
        Val::Single(n) if *n >= 10 => {
            *v1 = Val::Pair(Box::new((Val::Single(*n / 2), Val::Single((*n + 1) / 2))));
            Err(())
        }
        Val::Pair(p) => {
            split(&mut p.0)?;
            split(&mut p.1)
        }
        _ => Ok(()),
    }
}

fn main() {
    let data: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(|s| {
            let mut c = combinator::complete(parse_pair::<u32>);
            c(&s.unwrap()).unwrap().1
        })
        .collect();

    let res = data
        .iter()
        .enumerate()
        .cartesian_product(data.iter().enumerate())
        .filter_map(|((a, va), (b, vb))| {
            if a == b {
                None
            } else {
                Some(add_val(va.clone(), vb.clone()).magn())
            }
        })
        .max()
        .unwrap();
    eprintln!("{}", res);
}
