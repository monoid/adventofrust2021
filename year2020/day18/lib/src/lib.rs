use nom::{
    branch::alt,
    bytes::complete::tag,
    character::streaming::char,
    combinator::{complete, map},
    multi::many0,
    sequence::{delimited, pair, preceded},
    IResult,
};

fn parse_operand1(inp: &str) -> IResult<&str, i64> {
    alt((
        nom::character::complete::i64,
        delimited(char('('), calc_expression1, char(')')),
    ))(inp)
}

pub fn calc_expression1(inp: &str) -> IResult<&str, i64> {
    map(
        pair(
            parse_operand1,
            many0(complete(pair(
                delimited(char(' '), alt((char('+'), char('*'))), char(' ')),
                parse_operand1,
            ))),
        ),
        |(mut val, ops): (i64, Vec<(char, i64)>)| {
            for (op, val1) in ops {
                match op {
                    '+' => val += val1,
                    '*' => val *= val1,
                    _ => panic!("unexpected op"),
                }
            }
            val
        },
    )(inp)
}

fn parse_operand2(inp: &str) -> IResult<&str, i64> {
    alt((
        nom::character::complete::i64,
        delimited(char('('), calc_expression2, char(')')),
    ))(inp)
}

fn parse_sum2(inp: &str) -> IResult<&str, i64> {
    map(
        pair(
            parse_operand2,
            many0(complete(preceded(tag(" + "), parse_sum2))),
        ),
        |(val, ops)| val + ops.into_iter().sum::<i64>(),
    )(inp)
}

pub fn calc_expression2(inp: &str) -> IResult<&str, i64> {
    map(
        pair(
            parse_sum2,
            many0(complete(preceded(tag(" * "), parse_sum2))),
        ),
        |(val, ops)| val * ops.into_iter().product::<i64>(),
    )(inp)
}

#[cfg(test)]
mod tests {
    use crate::calc_expression1;

    #[test]
    fn test_num() {
        assert_eq!(calc_expression1("42").unwrap().1, 42);
        assert_eq!(calc_expression1("(42)").unwrap().1, 42);
        assert_eq!(calc_expression1("(((42)))").unwrap().1, 42);
    }

    #[test]
    fn test_plus() {
        assert_eq!(calc_expression1("41 + 1").unwrap().1, 42);
    }
}
