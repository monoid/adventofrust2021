use std::io::{self, BufRead as _};

pub type Map = Vec<Vec<u8>>;

pub fn read_map() -> io::Result<Map> {
    io::stdin()
        .lock()
        .lines()
        .map(|s| s.map(String::into_bytes))
        .collect()
}

fn swap(map: &mut Map, p1: (usize, usize), p2: (usize, usize)) {
    let v1 = map[p1.1][p1.0];
    let v2 = map[p2.1][p2.0];

    map[p1.1][p1.0] = v2;
    map[p2.1][p2.0] = v1;
}

fn advance_by_direction(map: &mut Map, kind: u8, dir: (usize, usize)) -> bool {
    let h = map.len();
    let w = map[0].len();
    let mut moves = Vec::new();

    for (y, l) in map.iter().enumerate() {
        for (x, p) in l.iter().cloned().enumerate() {
            if p == kind {
                let nx = (x + dir.0) % w;
                let ny = (y + dir.1) % h;
                if map[ny][nx] == b'.' {
                    moves.push(((x, y), (nx, ny)));
                }
            }
        }
    }

    let has_moves = !moves.is_empty();
    for (f, t) in moves.into_iter() {
        swap(map, f, t);
    }

    has_moves
}

pub fn advance(map: &mut Map) -> bool {
    // Vertical
    let m1 = advance_by_direction(map, b'>', (1, 0));
    let m2 = advance_by_direction(map, b'v', (0, 1));

    m1 | m2
}

pub fn print_map(map: &Map) {
    for l in map {
        for c in l.iter().cloned() {
            eprint!("{}", c as char);
        }
        eprintln!();
    }
    eprintln!();
}
