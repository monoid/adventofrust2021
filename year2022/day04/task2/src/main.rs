use std::ops::RangeInclusive;

fn main() {
    let lines = lib::read_lines();
    let ans = lines.filter(overlaps).count();
    println!("{}", ans);
}

fn overlaps((p1, p2): &(RangeInclusive<u8>, RangeInclusive<u8>)) -> bool {
    overlaps_range(p1, p2) || overlaps_range(p2, p1)
}

fn overlaps_range(p1: &RangeInclusive<u8>, p2: &RangeInclusive<u8>) -> bool {
    p1.contains(p2.start()) || p1.contains(p2.end())
}
