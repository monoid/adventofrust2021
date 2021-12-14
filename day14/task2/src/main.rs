use itertools::Itertools as _;
use std::{
    collections::HashMap,
    io::{self, BufRead as _},
};

type State = HashMap<(u8, u8), u64>;
fn from_init(init: &[u8]) -> State {
    let mut res = State::default();
    for p in init.iter().cloned().tuple_windows::<(_, _)>() {
        *res.entry(p).or_default() += 1;
    }
    res
}

fn advance(state: State, rules: &HashMap<(u8, u8), u8>) -> State {
    let mut res = State::default();
    for (p, cnt) in state.into_iter() {
        match rules.get(&p) {
            Some(&ins) => {
                *res.entry((p.0, ins)).or_default() += cnt;
                *res.entry((ins, p.1)).or_default() += cnt;
            }
            None => *res.entry(p).or_default() += cnt,
        }
    }
    res
}

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let mut lines = stdin.lines();
    let init_state = lines.next().unwrap().unwrap().into_bytes();
    lines.next().unwrap().unwrap();

    let mut state = from_init(&init_state);

    let mut rules = HashMap::<(u8, u8), u8>::new();
    for rule in lines {
        let rule = rule.unwrap().into_bytes();
        assert_eq!(rule.len(), 7);
        rules.insert((rule[0], rule[1]), rule[6]);
    }

    for _ in 0..40 {
        state = advance(state, &rules);
    }

    let mut hist = HashMap::<_, u64>::new();
    for ((_, b), cnt) in state.into_iter() {
        *hist.entry(b).or_default() += cnt;
    }
    *hist.entry(init_state[0]).or_default() += 1;

    println!(
        "{}",
        hist.values().max().unwrap() - hist.values().min().unwrap()
    );
}
