use nom::{
    bytes::complete::tag,
    character::{self, complete::alpha1},
    combinator::{complete, map},
    sequence::separated_pair,
    IResult,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Policy {
    min: usize,
    max: usize,
    c: char,
}

impl Policy {
    pub fn verify1(&self, pass: &str) -> bool {
        let c = self.c;
        let count = pass.chars().filter(|&a| a == c).count();
        self.min <= count && count <= self.max
    }

    pub fn verify2(&self, pass: &str) -> bool {
        let c = self.c;

        let c1 = pass.chars().nth(self.min - 1).unwrap();
        let c2 = pass.chars().nth(self.max - 1).unwrap();
        (c1 == c && c2 != c) || (c2 == c && c1 != c)
    }
}

pub fn parse_policy(line: &str) -> IResult<&str, Policy> {
    map(
        separated_pair(
            separated_pair(character::complete::u32, tag("-"), character::complete::u32),
            tag(" "),
            character::complete::anychar,
        ),
        |((min, max), c)| Policy {
            min: min as _,
            max: max as _,
            c,
        },
    )(line)
}

pub fn parse_line(line: &str) -> Result<(Policy, &str), ()> {
    complete(separated_pair(parse_policy, tag(": "), alpha1))(line)
        .map(|(_, res)| res)
        .map_err(|_| ())
}

#[cfg(test)]
mod tests {
    use crate::{parse_line, Policy};

    #[test]
    fn test_parse() {
        let (pol, pass) = parse_line("1-3 c: password").unwrap();
        assert_eq!(
            pol,
            Policy {
                min: 1,
                max: 3,
                c: 'c',
            }
        );

        assert_eq!(pass, "password");
    }

    #[test]
    fn test_policy_fail_min() {
        let pol = Policy {
            min: 2,
            max: 3,
            c: 'b',
        };
        assert!(!pol.verify1("abc"));
    }

    #[test]
    fn test_policy_min() {
        let pol = Policy {
            min: 1,
            max: 3,
            c: 'b',
        };
        assert!(pol.verify1("abc"));
    }

    #[test]
    fn test_policy_mid() {
        let pol = Policy {
            min: 1,
            max: 3,
            c: 'b',
        };
        assert!(pol.verify1("abbc"));
    }

    #[test]
    fn test_policy_max() {
        let pol = Policy {
            min: 1,
            max: 3,
            c: 'b',
        };
        assert!(pol.verify1("abbbc"));
    }

    #[test]
    fn test_policy_fail_max() {
        let pol = Policy {
            min: 1,
            max: 3,
            c: 'b',
        };
        assert!(!pol.verify1("abbbbc"));
    }

    #[test]
    fn test_verify2_1() {
        let pol = Policy {
            min: 5,
            max: 6,
            c: 'r',
        };
        assert!(!pol.verify2("rrrrcqr"));
    }

    #[test]
    fn test_verify2_2() {
        let pol = Policy {
            min: 1,
            max: 3,
            c: 'a',
        };
        assert!(pol.verify2("abcde"));
    }

    #[test]
    fn test_verify2_3() {
        let pol = Policy {
            min: 1,
            max: 3,
            c: 'b',
        };
        assert!(!pol.verify2("cdefg"));
    }

    #[test]
    fn test_verify2_4() {
        let pol = Policy {
            min: 2,
            max: 9,
            c: 'c',
        };
        assert!(!pol.verify2("ccccccccc"));
    }
}
