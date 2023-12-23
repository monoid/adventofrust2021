use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, one_of},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, pair, separated_pair},
};

#[derive(Debug, PartialEq)]
pub struct Workflow {
    pub name: String,
    pub rules: Vec<Rule>,
    pub else_workflow: String,
}

impl Workflow {
    pub fn apply(&self, item: &Item) -> &str {
        for rule in &self.rules {
            let item_val = *item.get(&rule.cat).unwrap();
            let matches = match rule.ord {
                '<' => item_val < rule.val,
                '>' => item_val > rule.val,
                _ => panic!("unkonw ord"),
            };
            if matches {
                return &rule.target;
            }
        }
        &self.else_workflow
    }

    pub fn parse(inp: &str) -> nom::IResult<&str, Workflow> {
        map(
            pair(
                alpha1,
                delimited(
                    tag("{"),
                    separated_pair(
                        separated_list1(tag(","), Rule::parse_rule),
                        tag(","),
                        alpha1,
                    ),
                    tag("}"),
                ),
            ),
            |(name, (rules, else_workflow))| Workflow {
                name: name.to_owned(),
                rules,
                else_workflow: else_workflow.to_owned(),
            },
        )(inp)
    }
}

#[derive(Debug, PartialEq)]
pub struct Rule {
    pub cat: char,
    pub ord: char,
    pub val: u32,
    pub target: String,
}

impl Rule {
    pub fn parse_rule(inp: &str) -> nom::IResult<&str, Rule> {
        map(
            separated_pair(
                pair(
                    pair(
                        one_of::<_, _, nom::error::Error<&str>>("amsx"),
                        one_of("<>"),
                    ),
                    nom::character::complete::u32,
                ),
                tag(":"),
                alpha1,
            ),
            |(((cat, ord), val), target)| Rule {
                cat,
                ord,
                val,
                target: target.to_owned(),
            },
        )(inp)
    }
}

pub type Item = HashMap<char, u32>;

pub fn parse_item(inp: &str) -> nom::IResult<&str, Item> {
    map(
        delimited(
            tag("{"),
            separated_list1(tag(","), parse_item_rule),
            tag("}"),
        ),
        |vec| vec.into_iter().collect(),
    )(inp)
}

fn parse_item_rule(inp: &str) -> nom::IResult<&str, (char, u32)> {
    separated_pair(one_of("amsx"), tag("="), nom::character::complete::u32)(inp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rule() {
        let rule = Rule::parse_rule("a<2006:qkq");
        assert_eq!(
            rule,
            Ok((
                "",
                Rule {
                    cat: 'a',
                    ord: '<',
                    val: 2006,
                    target: "qkq".to_owned()
                }
            ))
        );
    }

    #[test]
    fn test_workflow() {
        let inp = "px{a<2006:qkq,m>2090:A,rfg}";
        let workflow = Workflow::parse(inp);

        assert_eq!(
            workflow,
            Ok((
                "",
                Workflow {
                    name: "px".to_owned(),
                    rules: vec![
                        Rule {
                            cat: 'a',
                            ord: '<',
                            val: 2006,
                            target: "qkq".to_owned(),
                        },
                        Rule {
                            cat: 'm',
                            ord: '>',
                            val: 2090,
                            target: "A".to_owned(),
                        }
                    ],
                    else_workflow: "rfg".to_owned()
                }
            ))
        )
    }
}

pub fn read_data() -> (HashMap<String, Workflow>, Vec<Item>) {
    let mut workflows = HashMap::new();
    let mut items = vec![];

    let mut lines = std::io::stdin().lines().map(|line| line.unwrap());

    for line in &mut lines {
        if line.is_empty() {
            break;
        }
        let wf = Workflow::parse(line.trim()).unwrap().1;
        workflows.insert(wf.name.clone(), wf);
    }

    for line in &mut lines {
        items.push(parse_item(line.trim()).unwrap().1);
    }

    (workflows, items)
}

pub fn execute(workflows: &HashMap<String, Workflow>, item: &Item) -> bool {
    let mut rule = workflows.get("in").unwrap();
    loop {
        let target = rule.apply(item);
        match target {
            "R" => return false,
            "A" => return true,
            _ => rule = workflows.get(target).unwrap(),
        }
    }
}
