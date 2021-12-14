use itertools::Itertools as _;
use std::{
    collections::HashMap,
    io::{self, BufRead as _},
};

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let mut lines = stdin.lines();
    let mut state = lines.next().unwrap().unwrap().into_bytes();
    lines.next().unwrap().unwrap();

    let mut rules = HashMap::<[u8; 2], u8>::new();
    for rule in lines {
        let rule = rule.unwrap().into_bytes();
        assert_eq!(rule.len(), 7);
        rules.insert([rule[0], rule[1]], rule[6]);
    }

    for _ in 0..10 {
        assert!(!state.is_empty());
        let mut new_state = Vec::with_capacity(3 * state.len() / 2 + 1);
        new_state.push(state[0]);
        for (a, b) in state.iter().cloned().tuple_windows::<(_, _)>() {
            if let Some(ins) = rules.get(&[a, b]) {
                new_state.push(*ins);
            }
            new_state.push(b);
        }
        state = new_state;
    }

    let mut hist = HashMap::<_, usize>::new();
    for c in state {
        *hist.entry(c).or_default() += 1;
    }
    println!(
        "{}",
        hist.values().max().unwrap() - hist.values().min().unwrap()
    );
}
