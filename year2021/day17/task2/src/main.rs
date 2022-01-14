use std::{
    io::{self, Read},
    ops::RangeInclusive,
};

#[derive(Debug)]
struct Target {
    x: RangeInclusive<isize>,
    y: RangeInclusive<isize>,
}

impl Target {
    fn is_inside(&self, x: isize, y: isize) -> bool {
        self.x.contains(&x) && self.y.contains(&y)
    }

    fn too_much(&self, x: isize) -> bool {
        self.x.end() < &x
    }

    fn overthrow(&self, y: isize) -> bool {
        self.y.end() < &y
    }
}

fn parse_input(inp: &str) -> Target {
    let re =
        regex::Regex::new(r"^target area: x=([-0-9]+)\.\.([-0-9]+), y=([-0-9]+)\.\.([-0-9]+)$")
            .unwrap();
    let ca = re.captures(inp).unwrap();
    let x1: isize = ca.get(1).unwrap().as_str().parse().unwrap();
    let x2: isize = ca.get(2).unwrap().as_str().parse().unwrap();
    let y1: isize = ca.get(3).unwrap().as_str().parse().unwrap();
    let y2: isize = ca.get(4).unwrap().as_str().parse().unwrap();

    Target {
        x: x1..=x2,
        y: y1..=y2,
    }
}

fn trajectory(sx: isize, sy: isize) -> impl Iterator<Item = (isize, isize, isize, isize)> {
    std::iter::successors(Some((0, 0, sx, sy)), |&(px, py, sx, sy)| {
        Some((px + sx, py + sy, sx - sx.signum(), sy - 1))
    })
}

fn main() {
    let mut inp = String::new();
    io::stdin().lock().read_to_string(&mut inp).unwrap();

    let sq = parse_input(inp.trim());

    let mut count = 0;

    for vx in 1..=*sq.x.end() {
        let dy = sq.y.end() - sq.y.start() + 1;
        'yloop:
        // These ranges are somewhat artificial.
        for vy in *sq.y.start()..*sq.y.end() + 8*dy {
            for (px, py, sx, _sy) in trajectory(vx, vy) {
                if sq.is_inside(px, py) {
                    count += 1;
                    break;
                } else if sq.too_much(px) || sq.y.start() > &py
                || (sx == 0 && &px < sq.x.start()){
                    if sq.overthrow(py) {
                        break 'yloop;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    println!("{}", count);
}
