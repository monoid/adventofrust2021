use std::ops::RangeInclusive;

pub fn parse_line(inp: &str) -> nom::IResult<&str, (RangeInclusive<u8>, RangeInclusive<u8>)> {
    use nom::bytes::complete::tag;
    use nom::combinator::all_consuming;
    use nom::sequence::separated_pair;

    all_consuming(separated_pair(parse_range, tag(","), parse_range))(inp)
}

fn parse_range(inp: &str) -> nom::IResult<&str, RangeInclusive<u8>> {
    use nom::bytes::complete::tag;
    use nom::character::complete::u8 as parse_u8;
    use nom::combinator::map;
    use nom::sequence::separated_pair;

    map(separated_pair(parse_u8, tag("-"), parse_u8), |(p1, p2)| {
        p1..=p2
    })(inp)
}

pub fn read_lines() -> impl Iterator<Item = (RangeInclusive<u8>, RangeInclusive<u8>)> {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    stdin
        .lock()
        .lines()
        .map(|r| parse_line(&r.unwrap()).unwrap().1)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
