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

    let mut global_max_height = isize::MIN;

    for st_x in 1..*sq.x.end() {
        'yloop: for st_y in 0..(8 * (sq.y.end() - sq.y.start() + 1)) {
            let mut traj_max_height = isize::MIN;
            for (px, py, sx, _sy) in trajectory(st_x, st_y) {
                traj_max_height = std::cmp::max(traj_max_height, py);
                if sq.is_inside(px, py) {
                    global_max_height = std::cmp::max(global_max_height, traj_max_height);
                    break;
                } else if sq.too_much(px) || sq.y.start() > &py || (sx == 0 && &px < sq.x.start()) {
                    if sq.overthrow(py) {
                        break 'yloop;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    println!("{}", global_max_height);
}
