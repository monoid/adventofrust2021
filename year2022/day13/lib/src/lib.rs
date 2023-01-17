use itertools::Itertools;
use std::cmp::Ordering;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Node {
    Number(u32),
    List(Vec<Node>),
}

impl Node {
    pub fn compare(upper: &Node, lower: &Node) -> Ordering {
        match (upper, lower) {
            (Node::Number(a), Node::Number(b)) => a.cmp(b),
            (Node::Number(a), Node::List(_)) => {
                Node::compare(&Node::List(vec![Node::Number(*a)]), lower)
            }
            (Node::List(_), Node::Number(b)) => {
                Node::compare(upper, &Node::List(vec![Node::Number(*b)]))
            }
            (Node::List(alist), Node::List(blist)) => {
                for zip in alist.iter().zip_longest(blist) {
                    match zip {
                        itertools::EitherOrBoth::Both(a, b) => match Node::compare(a, b) {
                            Ordering::Less => return Ordering::Less,
                            Ordering::Equal => continue,
                            Ordering::Greater => return Ordering::Greater,
                        },

                        itertools::EitherOrBoth::Left(_) => return Ordering::Greater,
                        itertools::EitherOrBoth::Right(_) => return Ordering::Less,
                    }
                }
                Ordering::Equal
            }
        }
    }
}

pub fn parse_node(inp: &str) -> nom::IResult<&str, Node> {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::u32;
    use nom::combinator::map;
    use nom::multi::separated_list0;
    use nom::sequence::delimited;

    alt((
        map(u32, Node::Number),
        map(
            delimited(tag("["), separated_list0(tag(","), parse_node), tag("]")),
            Node::List,
        ),
    ))(inp)
}

pub fn read() -> Vec<(Node, Node)> {
    let mut val = vec![];

    let mut stdin = std::io::stdin().lines();
    while let Some(res1) = stdin.next() {
        let line1 = res1.unwrap();
        let line2 = stdin.next().unwrap().unwrap();
        val.push((parse_node(&line1).unwrap().1, parse_node(&line2).unwrap().1));

        if stdin.next().is_none() {
            break;
        }
    }

    val
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use crate::{parse_node, Node};

    #[test]
    fn test_empty() {
        let r = parse_node("[]");
        assert_eq!(r, Ok(("", Node::List(vec![]))));
    }

    #[test]
    fn test_number() {
        let r = parse_node("[3]");
        assert_eq!(r, Ok(("", Node::List(vec![Node::Number(3)]))));
    }

    #[test]
    fn test_mixed() {
        let r = parse_node("[[4],3]");
        assert_eq!(
            r,
            Ok((
                "",
                Node::List(vec![Node::List(vec![Node::Number(4)]), Node::Number(3)])
            ))
        );
    }

    #[test]
    fn test_compare1() {
        let n1 = parse_node("[1,1,3,1,1]").unwrap().1;
        let n2 = parse_node("[1,1,5,1,1]").unwrap().1;
        assert_eq!(Node::compare(&n1, &n2,), Ordering::Less);
    }

    #[test]
    fn test_compare2() {
        let n1 = parse_node("[[1],[2,3,4]]").unwrap().1;
        let n2 = parse_node("[[1],4]").unwrap().1;

        assert_eq!(Node::compare(&n1, &n2), Ordering::Less);
    }

    #[test]
    fn test_compare3() {
        let n1 = parse_node("[9]").unwrap().1;
        let n2 = parse_node("[[8,7,6]]").unwrap().1;

        assert_eq!(Node::compare(&n1, &n2), Ordering::Greater);
    }

    #[test]
    fn test_compare4() {
        let n1 = parse_node("[[4,4],4,4]").unwrap().1;
        let n2 = parse_node("[[4,4],4,4,4]").unwrap().1;

        assert_eq!(Node::compare(&n1, &n2), Ordering::Less);
    }

    #[test]
    fn test_compare5() {
        let n1 = parse_node("[[[]]]").unwrap().1;
        let n2 = parse_node("[[]]").unwrap().1;

        assert_eq!(Node::compare(&n1, &n2), Ordering::Greater);
    }
}
