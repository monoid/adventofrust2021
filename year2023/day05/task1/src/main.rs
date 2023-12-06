fn main() {
    let (seeds, maps) = lib::read_scene();
    let result = seeds
        .into_iter()
        .map(|seed| maps.iter().fold(seed, |n, map| map.transform(n)))
        .min()
        .unwrap();
    println!("{:?}", result);
}
