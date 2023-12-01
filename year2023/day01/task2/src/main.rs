use nom::{
    combinator::{peek, recognize},
    sequence::{pair, terminated},
};

fn main() {
    let result: u32 = std::io::stdin()
        .lines()
        .filter(|l| !l.as_ref().unwrap().is_empty())
        .map(|l| parse_input(&l.unwrap()))
        .sum();
    eprintln!("{}", result);
}

fn parse_input(inp: &str) -> u32 {
    let digits = calib_parser(inp).unwrap().1;
    let first = *digits.first().unwrap();
    let last = *digits.last().unwrap();

    (10 * first + last) as _
}

fn calib_char_parser(inp: &str) -> nom::IResult<&str, Option<u8>> {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{anychar, satisfy};
    use nom::combinator::map;
    use nom::combinator::value;

    alt((
        map(satisfy(|c| char::is_digit(c, 10)), |c| {
            Some((c as u8) - b'0')
        }),
        // First, try to parse a textual digit without consuming it,
        // then consume only single char.
        //
        // Looks hackish, but works.
        terminated(
            peek(alt((
                value(Some(1), tag("one")),
                value(Some(2), tag("two")),
                value(Some(3), tag("three")),
                value(Some(4), tag("four")),
                value(Some(5), tag("five")),
                value(Some(6), tag("six")),
                value(Some(7), tag("seven")),
                value(Some(8), tag("eight")),
                value(Some(9), tag("nine")),
            ))),
            anychar,
        ),
        value(None, anychar),
    ))(inp)
}

fn calib_parser(inp: &str) -> nom::IResult<&str, Vec<u8>> {
    use nom::combinator::map;
    use nom::multi::many0;

    map(many0(calib_char_parser), |elements| {
        elements.into_iter().filter_map(|n| n).collect()
    })(inp)
}
