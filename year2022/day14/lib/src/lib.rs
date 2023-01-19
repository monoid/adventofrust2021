use std::collections::HashSet;

use itertools::Itertools;

const SOURCE_X: i32 = 500;

pub struct Map {
    map: HashSet<(u32, i32)>,
    xs: (i32, i32),
    maxy: u32,
}

#[derive(Debug, Clone, Copy)]
enum Particle {
    Stabilized(i32, u32),
    Fallen(i32, u32),
}

impl Map {
    pub fn read() -> Self {
        let data = read();
        let xs = data
            .iter()
            .flatten()
            .cloned()
            .map(|(x, _)| x)
            .minmax()
            .into_option()
            .unwrap();
        let mut ys = data
            .iter()
            .flatten()
            .cloned()
            .map(|(_, y)| y)
            .minmax()
            .into_option()
            .unwrap();
        //
        ys.0 = 0;

        let mut map = HashSet::<(u32, i32)>::default();
        for path in data {
            for (p1, p2) in path.iter().cloned().tuple_windows() {
                if p1.0 == p2.0 {
                    let mut y1 = p1.1;
                    let mut y2 = p2.1;
                    if y1 > y2 {
                        std::mem::swap(&mut y1, &mut y2);
                    }
                    for y in y1..=y2 {
                        map.insert((y, p1.0 - xs.0));
                    }
                } else if p1.1 == p2.1 {
                    let mut x1 = p1.0;
                    let mut x2 = p2.0;
                    if x1 > x2 {
                        std::mem::swap(&mut x1, &mut x2);
                    }
                    for x in x1..=x2 {
                        map.insert((p1.1, x - xs.0));
                    }
                } else {
                    panic!("General lines are not expected");
                }
            }
        }

        Self {
            map,
            xs,
            maxy: ys.1,
        }
    }

    pub fn eval1(&mut self) -> u32 {
        let mut cnt = 0;

        while let Particle::Stabilized(_, _) = self.run_particle() {
            cnt += 1;
        }
        cnt
    }

    pub fn eval2(&mut self) -> u32 {
        let mut cnt = 0;

        while !self.map.contains(&(0, SOURCE_X - self.xs.0)) {
            if let Particle::Fallen(x, y) = self.run_particle() {
                self.map.insert((y, x));
            }
            cnt += 1;
        }
        cnt
    }

    fn run_particle(&mut self) -> Particle {
        // Dropping a new point
        let mut pt = (SOURCE_X - self.xs.0, 0u32);
        loop {
            let pt_down = (pt.0, pt.1 + 1);
            let pt_left = (pt.0 - 1, pt.1 + 1);
            let pt_right = (pt.0 + 1, pt.1 + 1);

            if !self.map.contains(&(pt_down.1, pt_down.0)) {
                pt = pt_down;
                if pt.1 > self.maxy {
                    break Particle::Fallen(pt.0, pt.1);
                }
                continue;
            }

            if !self.map.contains(&(pt_left.1, pt_left.0)) {
                pt = pt_left;
                if pt.1 > self.maxy {
                    break Particle::Fallen(pt.0, pt.1);
                }
                continue;
            }

            if !self.map.contains(&(pt_right.1, pt_right.0)) {
                pt = pt_right;
                if pt.1 > self.maxy {
                    break Particle::Fallen(pt.0, pt.1);
                }
                continue;
            }

            if pt.1 > self.maxy {
                break Particle::Fallen(pt.0, pt.1);
            }
            assert!(!self.map.contains(&(pt.1, pt.0)));
            self.map.insert((pt.1, pt.0));
            // Stabilized
            break Particle::Stabilized(pt.0, pt.1);
        }
    }
}

pub fn read() -> Vec<Vec<(i32, u32)>> {
    std::io::stdin()
        .lines()
        .map(|r| parse_stone_path(&r.unwrap()).unwrap().1)
        .collect()
}

pub fn parse_stone_path(inp: &str) -> nom::IResult<&str, Vec<(i32, u32)>> {
    use nom::bytes::complete::tag;
    use nom::character::complete::{i32, u32};
    use nom::multi::separated_list1;
    use nom::sequence::separated_pair;

    separated_list1(tag(" -> "), separated_pair(i32, tag(","), u32))(inp)
}
