use std::{cmp::Reverse, collections::BinaryHeap};

const TOP: usize = 3;

fn main() {
    let lines = lib::lines().map(Result::unwrap);
    let elves = lib::elves(lines).into_iter();
    let mut heap = BinaryHeap::default();
    for elf in elves {
        let w = elf.calories();
        heap.push(Reverse(w));
        if heap.len() > TOP {
            heap.pop();
        }
    }
    println!("{}", heap.into_iter().map(|v| v.0).sum::<u32>());
}
