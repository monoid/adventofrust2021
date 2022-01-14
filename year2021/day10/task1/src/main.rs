use std::io::{self, BufRead as _};

/** Return invalid char if line is not valid. */
fn find_illegal(v: &str) -> Option<char> {
    let mut stack = vec![];
    for c in v.chars() {
        match c {
            '{' | '[' | '(' | '<' => stack.push(c),
            '}' | ']' | ')' | '>' => {
                let op = stack.pop().expect("Unbalanced");
                if closing(op) != c {
                    return Some(c);
                }
            }
            _ => {}
        }
    }
    None
}

fn closing(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => unreachable!(),
    }
}

fn char_cost(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn main() {
    let score = io::stdin()
        .lock()
        .lines()
        .filter_map(|s| find_illegal(&s.unwrap()))
        .map(char_cost)
        .sum::<u32>();
    println!("{}", score);
}
