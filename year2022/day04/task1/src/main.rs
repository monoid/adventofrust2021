use std::ops::RangeInclusive;

fn main() {
    let lines = lib::read_lines();
    let ans = lines.filter(fully_contains).count();
    println!("{}", ans);
}

fn fully_contains((p1, p2): &(RangeInclusive<u8>, RangeInclusive<u8>)) -> bool {
    contains_range(p1, p2) || contains_range(p2, p1)
}

fn contains_range(p1: &RangeInclusive<u8>, p2: &RangeInclusive<u8>) -> bool {
    p1.contains(p2.start()) && p1.contains(p2.end())
}
