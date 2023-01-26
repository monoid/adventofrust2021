fn main() {
    const SIZE: i32 = 4000000;

    let mut data = vec![];
    for r in std::io::stdin().lines() {
        let line = r.unwrap();
        let (sensor, beacon) = lib::parse_line(&line).unwrap().1;
        data.push((sensor, beacon));
    }

    for row_num in 0..SIZE {
        let mut ranges = lib::intree::RangeSet::default();

        for (sensor, beacon) in &data {
            if let Some((x1, x2)) = lib::intersects_with(*sensor, *beacon, row_num) {
                ranges.insert(x1..=x2);
            }
            // Well, actually, it is covered, isn't it?
            if beacon.1 == row_num {
                ranges.insert(beacon.1..=beacon.1);
            }
        }

        let target = lib::intree::LRange(0..=SIZE);
        let ranges_we_need: Vec<_> = ranges
            .into_inner()
            .into_iter()
            .filter(|range| range.intersects(&target))
            .collect();
        if ranges_we_need.len() > 1 {
            if ranges_we_need.len() == 2 {
                let max = ranges_we_need[0].end() + 1;
                let min = ranges_we_need[1].start() - 1;
                if min == max {
                    println!("{}", (min as u64) * (SIZE as u64) + (row_num as u64));
                } else {
                    panic!("Unexpected ranges: {:?}", ranges_we_need);
                }
            } else {
                panic!("Too many points: {:?}", ranges_we_need);
            }
        }
    }
}
