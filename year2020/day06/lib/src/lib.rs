use itertools::Itertools;
use std::io;

pub fn records<R: io::BufRead>(input: R) -> impl Iterator<Item = String> {
    let data = input.lines().map(Result::unwrap).collect_vec();

    data.into_iter().batching(|it| {
        let mut buf = String::new();
        loop {
            match it.next() {
                None => {
                    if buf.is_empty() {
                        return None;
                    } else {
                        return Some(buf);
                    }
                }
                Some(other) => {
                    if other.is_empty() {
                        return Some(buf);
                    } else {
                        if !buf.is_empty() {
                            buf.push(' ');
                        }
                        buf.push_str(&other);
                    }
                }
            }
        }
    })
}
