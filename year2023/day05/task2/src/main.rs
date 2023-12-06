use itertools::Itertools;

fn main() {
    let (seeds, maps) = lib::read_scene();
    let mut mins = vec![];
    for mut c in &seeds.into_iter().chunks(2) {
        let base = c.next().unwrap();
        let len = c.next().unwrap();
        let ranges = vec![(base..=(base + len - 1))];
        let mut output = maps.iter().fold(ranges, |r, map| map.transform_ranges(&r));
        output.sort_unstable_by_key(|range| *range.start());

        assert!(!output[0].is_empty());
        mins.push(*output[0].start());
    }

    let min = mins.into_iter().min().unwrap();

    println!("{}", min);
}
