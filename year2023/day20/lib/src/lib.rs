use std::collections::HashMap;
use std::collections::VecDeque;
use std::rc::Rc;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::combinator::all_consuming;
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::sequence::preceded;
use nom::sequence::separated_pair;

pub enum Node {
    Broadcast(Rc<str>, Vec<Rc<str>>),
    FlipFlop(Rc<str>, bool, Vec<Rc<str>>),
    Conj(Rc<str>, HashMap<Rc<str>, bool>, Vec<Rc<str>>),
}

impl Node {
    pub fn parse(inp: &str) -> nom::IResult<&str, (Rc<str>, Node)> {
        alt((
            map(
                separated_pair(
                    tag("broadcaster"),
                    tag(" -> "),
                    separated_list0(tag(", "), alpha1),
                ),
                |(name, targets): (&str, _)| {
                    let name: Rc<str> = name.into();
                    (
                        name.clone(),
                        Node::Broadcast(name, targets.into_iter().map(Into::into).collect()),
                    )
                },
            ),
            map(
                separated_pair(
                    preceded(tag("&"), alpha1),
                    tag(" -> "),
                    separated_list0(tag(", "), alpha1),
                ),
                |(name, targets): (&str, _)| {
                    let name: Rc<str> = name.into();
                    (
                        name.clone(),
                        Node::Conj(
                            name,
                            <_>::default(),
                            targets.into_iter().map(Into::into).collect(),
                        ),
                    )
                },
            ),
            map(
                separated_pair(
                    preceded(tag("%"), alpha1),
                    tag(" -> "),
                    separated_list0(tag(", "), alpha1),
                ),
                |(name, targets): (&str, _)| {
                    let name: Rc<str> = name.into();
                    (
                        name.clone(),
                        Node::FlipFlop(name, false, targets.into_iter().map(Into::into).collect()),
                    )
                },
            ),
        ))(inp)
    }

    pub fn send_signal(
        &mut self,
        source: Rc<str>,
        level: bool,
    ) -> Box<dyn Iterator<Item = (Rc<str>, Rc<str>, bool)> + '_> {
        match self {
            Node::Broadcast(name, targets) => Box::new(
                targets
                    .iter()
                    .map(move |tgt| (name.clone(), tgt.clone(), level)),
            ),
            Node::FlipFlop(name, state, targets) => {
                if level {
                    Box::new(std::iter::empty())
                } else {
                    *state = !*state;
                    Box::new(
                        targets
                            .iter()
                            .map(move |tgt| (name.clone(), tgt.clone(), *state)),
                    )
                }
            }
            Node::Conj(name, state, targets) => {
                state.insert(source, level);
                let output = !state.values().all(|x| *x);

                Box::new(
                    targets
                        .iter()
                        .map(move |tgt| (name.clone(), tgt.clone(), output)),
                )
            }
        }
    }

    pub fn targets(&self) -> &[Rc<str>] {
        match self {
            Node::Broadcast(_, targets) => targets,
            Node::FlipFlop(_, _, targets) => targets,
            Node::Conj(_, _, targets) => targets,
        }
    }
}

fn link_conjunctions(nodes: &mut HashMap<Rc<str>, Node>) {
    let sources: Vec<_> = nodes
        .iter()
        .map(|(key, node)| (key.clone(), node.targets().to_owned()))
        .collect();
    for (src, tgts) in sources {
        for tgt in tgts {
            if let Some(Node::Conj(_, incoming, _)) = nodes.get_mut(&tgt) {
                incoming.insert(src.clone(), false);
            }
        }
    }
}

pub fn execute(nodes: &mut HashMap<Rc<str>, Node>) -> (u32, u32) {
    let mut active = 0;
    let mut passive = 0;
    let mut queue = VecDeque::<(Rc<str>, Rc<str>, bool)>::new();
    queue.push_back(("button".into(), "broadcaster".into(), false));

    while let Some((src, tgt, level)) = queue.pop_front() {
        if level {
            active += 1;
        } else {
            passive += 1;
        }
        if let Some(tgt_node) = nodes.get_mut(&tgt) {
            queue.extend(tgt_node.send_signal(src, level));
        }
    }

    (active, passive)
}

pub fn read_data() -> HashMap<Rc<str>, Node> {
    let mut nodes = std::io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let v = all_consuming(Node::parse)(&line).unwrap();
            v.1
        })
        .collect();
    link_conjunctions(&mut nodes);
    nodes
}
