use std::io::{self, BufRead as _};

fn find_closing_cost(v: &str) -> Option<u64> {
    let mut stack = vec![];
    for c in v.chars() {
        match c {
            '{' | '[' | '(' | '<' => stack.push(c),
            '}' | ']' | ')' | '>' => {
                let op = stack.pop().expect("Unbalanced");
                if closing(op).0 != c {
                    return None;
                }
            }
            _ => {}
        }
    }

    Some(
        stack
            .iter()
            .rev()
            .fold(0, |prev, c| 5 * prev + closing(*c).1),
    )
}

fn closing(c: char) -> (char, u64) {
    match c {
        '(' => (')', 1),
        '[' => (']', 2),
        '{' => ('}', 3),
        '<' => ('>', 4),
        _ => unreachable!(),
    }
}

fn main() {
    let mut score: Vec<u64> = io::stdin()
        .lock()
        .lines()
        .filter_map(|s| find_closing_cost(&s.unwrap()))
        .collect();
    score.sort_unstable();
    println!("{}", score[score.len() / 2]);
}
