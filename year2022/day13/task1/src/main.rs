use std::cmp::Ordering;

fn main() {
    let data = lib::read();
    let value = data
        .iter()
        .enumerate()
        .filter_map(|(i, (a, b))| {
            if lib::Node::compare(a, b) != Ordering::Greater {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum::<usize>();
    println!("{}", value);
}
