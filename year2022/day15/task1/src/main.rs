use std::collections::HashSet;

fn main() {
    const ROW_NUM: i32 = 2000000;

    let mut points = HashSet::<i32>::new();
    let mut beacons = HashSet::<i32>::new();

    for r in std::io::stdin().lines() {
        let line = r.unwrap();
        let (sensor, beacon) = lib::parse_line(&line).unwrap().1;
        if let Some((x1, x2)) = lib::intersects_with(sensor, beacon, ROW_NUM) {
            for x in x1..=x2 {
                points.insert(x);
            }
        }
        if beacon.1 == ROW_NUM {
            beacons.insert(beacon.0);
        }
    }
    println!("{}", (&points - &beacons).len());
}
