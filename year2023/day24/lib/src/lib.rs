use std::str::FromStr;

use nom::bytes::complete::tag;
use nom::sequence::separated_pair;
use nom::IResult;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Vector(pub ((i64, i64), i64));

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Point {
    pub pos: Vector,
    pub velocity: Vector,
}

fn parse_vector(inp: &str) -> IResult<&str, Vector, ()> {
    use nom::character::complete::i64 as parse_i64;
    use nom::combinator::map;

    map(
        separated_pair(
            separated_pair(parse_i64, tag(", "), parse_i64),
            tag(", "),
            parse_i64,
        ),
        Vector,
    )(inp)
}

fn parse_point(inp: &str) -> IResult<&str, Point, ()> {
    use nom::combinator::map;

    map(
        separated_pair(parse_vector, tag(" @ "), parse_vector),
        |(pos, velocity)| Point { pos, velocity },
    )(inp)
}

impl FromStr for Point {
    type Err = nom::Err<()>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use nom::combinator::all_consuming;

        all_consuming(parse_point)(s).map(|(_, v)| v)
    }
}

pub fn read_data() -> Vec<Point> {
    std::io::stdin()
        .lines()
        .map(|r| {
            let l = r.unwrap();
            Point::from_str(l.trim()).unwrap()
        })
        .collect()
}

// We have lines in the form (x, y) = (x0, y0) + t*(xv, yv).  For each point
// we have to find t and check that both are positive; or that they are in correct
// semiplane.
pub fn intersect_xy_forward(_p1: &Point, _p2: &Point) -> Option<(f64, f64)> {
    todo!()
}
