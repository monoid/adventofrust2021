fn main() {
    let univ = lib::read_data();
    let mut dist = 0u64;
    for gidx in 0..(univ.map.len() - 1) {
        let gal = univ.map[gidx];
        let others = &univ.map[gidx + 1..];
        for other in others {
            dist += univ.dist(gal, *other, 1);
        }
    }
    println!("{}", dist);
}
