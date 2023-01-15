use std::{cell::RefCell, collections::HashMap, io::Read};

use nom::{
    self,
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt, value},
    multi::separated_list0,
    sequence::{delimited, pair, preceded, separated_pair, terminated},
    IResult,
};

pub struct Monkey {
    pub id: i32,
    pub decl: MonkeyDecl,
    pub counter: usize,
}

impl Monkey {
    pub fn do_turn1(&mut self, data: &HashMap<i32, RefCell<Monkey>>) {
        for worry_level in self.decl.starting.drain(..) {
            let worry_level = self.decl.operation.eval(worry_level) / 3;
            let next_id = if worry_level % self.decl.test.divisible == 0 {
                self.decl.test.true_id
            } else {
                self.decl.test.false_id
            };
            let next_ref = data.get(&next_id).expect("Unknown next id");
            let mut next = next_ref.borrow_mut();
            next.decl.starting.push(worry_level);

            self.counter += 1;
        }
    }

    pub fn do_turn2(&mut self, data: &HashMap<i32, RefCell<Monkey>>, lcm: i64) {
        for worry_level in self.decl.starting.drain(..) {
            let worry_level = self.decl.operation.eval(worry_level) % lcm;
            let next_id = if worry_level % self.decl.test.divisible == 0 {
                self.decl.test.true_id
            } else {
                self.decl.test.false_id
            };
            let next_ref = data.get(&next_id).expect("Unknown next id");
            let mut next = next_ref.borrow_mut();
            next.decl.starting.push(worry_level);

            self.counter += 1;
        }
    }
}

pub struct MonkeyDecl {
    pub starting: Vec<i64>,
    pub operation: Expr,
    pub test: Test,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    Old,
    Number(i64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn eval(&self, old: i64) -> i64 {
        match self {
            Expr::Old => old,
            Expr::Number(n) => *n,
            Expr::Add(a, b) => a.eval(old) + b.eval(old),
            Expr::Sub(a, b) => a.eval(old) - b.eval(old),
            Expr::Mul(a, b) => a.eval(old) * b.eval(old),
            Expr::Div(a, b) => a.eval(old) / b.eval(old),
        }
    }
}

pub struct Test {
    pub divisible: i64,
    pub true_id: i32,
    pub false_id: i32,
}

pub fn parse_monkeys(inp: &str) -> IResult<&str, Vec<Monkey>> {
    use nom::combinator::all_consuming;

    all_consuming(separated_list0(tag("\n"), parse_monkey))(inp)
}

fn parse_monkey(inp: &str) -> IResult<&str, Monkey> {
    map(
        separated_pair(parse_monkey_id, tag("\n"), parse_monkey_content),
        |(id, decl)| Monkey {
            id,
            decl,
            counter: 0,
        },
    )(inp)
}

fn parse_monkey_id(inp: &str) -> IResult<&str, i32> {
    use nom::character::complete::i32;

    delimited(tag("Monkey "), i32, tag(":"))(inp)
}

fn parse_monkey_content(inp: &str) -> IResult<&str, MonkeyDecl> {
    map(
        separated_pair(parse_starting_items, tag("\n"), parse_operation_and_test),
        |(starting, (operation, test))| MonkeyDecl {
            starting,
            operation,
            test,
        },
    )(inp)
}

fn parse_starting_items(inp: &str) -> IResult<&str, Vec<i64>> {
    use nom::character::complete::i64;
    preceded(tag("  Starting items: "), separated_list0(tag(", "), i64))(inp)
}

fn parse_operation_and_test(inp: &str) -> IResult<&str, (Expr, Test)> {
    separated_pair(parse_expr_line, tag("\n"), parse_test)(inp)
}

fn parse_expr_line(inp: &str) -> IResult<&str, Expr> {
    preceded(tag("  Operation: new = "), parse_expr)(inp)
}

fn parse_test(inp: &str) -> IResult<&str, Test> {
    use nom::character::complete::i64;

    map(
        separated_pair(
            preceded(tag("  Test: divisible by "), i64),
            tag("\n"),
            parse_if_true_false,
        ),
        |(divisible, (true_id, false_id))| Test {
            divisible,
            true_id,
            false_id,
        },
    )(inp)
}

fn parse_expr(inp: &str) -> IResult<&str, Expr> {
    map(
        pair(
            parse_val,
            opt(pair(alt((tag(" + "), tag(" * "))), parse_val)),
        ),
        |(val1, rest)| match rest {
            None => val1,
            Some((" + ", val2)) => Expr::Add(Box::new(val1), Box::new(val2)),
            Some((" * ", val2)) => Expr::Mul(Box::new(val1), Box::new(val2)),
            Some((" - ", val2)) => Expr::Sub(Box::new(val1), Box::new(val2)),
            Some((" / ", val2)) => Expr::Div(Box::new(val1), Box::new(val2)),
            Some((op, _)) => panic!("uknonwn operator {:?}", op),
        },
    )(inp)
}

fn parse_val(inp: &str) -> IResult<&str, Expr> {
    use nom::character::complete::i64;

    alt((value(Expr::Old, tag("old")), map(i64, Expr::Number)))(inp)
}

fn parse_if_true_false(inp: &str) -> IResult<&str, (i32, i32)> {
    use nom::character::complete::i32;

    terminated(
        separated_pair(
            preceded(tag("    If true: throw to monkey "), i32),
            tag("\n"),
            preceded(tag("    If false: throw to monkey "), i32),
        ),
        tag("\n"),
    )(inp)
}

pub fn read_data() -> HashMap<i32, RefCell<Monkey>> {
    use std::io;

    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();

    parse_monkeys(&buf)
        .expect("Failed to parse input data")
        .1
        .into_iter()
        .map(|monkey| (monkey.id, monkey.into()))
        .collect()
}

// Bit slow, but works.
// https://en.wikipedia.org/wiki/Least_common_multiple#Using_a_simple_algorithm
pub fn lcm(orig_values: &[i64]) -> i64 {
    let mut values = orig_values.to_owned();
    loop {
        let mut min: Option<(usize, i64)> = None;
        let mut max = None;
        for (i, val) in values.iter().cloned().enumerate() {
            match min.as_mut() {
                Some(pair) => {
                    if pair.1 > val {
                        *pair = (i, val);
                    }
                }
                None => {
                    min = Some((i, val));
                }
            }
            match max.as_mut() {
                Some(m) => {
                    if *m < val {
                        *m = val;
                    }
                }
                None => {
                    max = Some(val);
                }
            }
        }

        if min.map(|(_, v)| v) == max {
            return min.unwrap().1;
        } else {
            let pos = min.unwrap().0;
            values[pos] += orig_values[pos];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_if_true_false() {
        let v = parse_if_true_false(
            "    If true: throw to monkey 2\n    If false: throw to monkey 1\n",
        );
        assert_eq!(v, Ok(("", (2, 1))));
    }

    #[test]
    fn test_operation_old() {
        let v = parse_expr_line("  Operation: new = old");
        assert_eq!(v, Ok(("", Expr::Old)), "{:?}", v);
    }

    #[test]
    fn test_operation_old_plus_old() {
        let v = parse_expr_line("  Operation: new = old + old");
        assert_eq!(
            v,
            Ok(("", Expr::Add(Box::new(Expr::Old), Box::new(Expr::Old)))),
            "{:?}",
            v,
        );
    }

    #[test]
    fn test_operation_old_mul_value() {
        let v = parse_expr_line("  Operation: new = old * 42");
        assert_eq!(
            v,
            Ok((
                "",
                Expr::Mul(Box::new(Expr::Old), Box::new(Expr::Number(42))),
            )),
            "{:?}",
            v,
        );
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(&vec![42, 8, 13]), 42 * 13 * 4);
    }
}
