use std::io;

fn count_taken((y, x): (usize, usize), map: &lib::Map) -> usize {
    let h = map.len() as isize;
    let w = map[0].len() as isize;

    [-1isize, 0, 1]
        .into_iter()
        .flat_map(|dy| [-1isize, 0, 1].into_iter().map(move |dx| (dy, dx)))
        .filter_map(|(dy, dx)| {
            if dy == 0 && dx == 0 {
                None
            } else {
                let sy = (y as isize) + dy;
                let sx = (x as isize) + dx;
                // It can be expressed with the monad Maybe...
                // usize: TryFrom<isize>
                // Vec::get(n) -> Option<...>
                // etc.
                // Or, filter by out-of-bounds coordinate can be lifted up.
                if (sy < 0) || (sy >= h) || (sx < 0) || (sx >= w) {
                    None
                } else {
                    Some((sy as usize, sx as usize))
                }
            }
        })
        .filter(|&(sy, sx)| map[sy][sx] == lib::State::Occupied)
        .count()
}
fn new_map(map: &lib::Map) -> lib::Map {
    map.iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .cloned()
                .enumerate()
                .map(|(x, state)| match state {
                    lib::State::Floor => lib::State::Floor,
                    lib::State::Empty => {
                        if count_taken((y, x), map) == 0 {
                            lib::State::Occupied
                        } else {
                            state
                        }
                    }
                    lib::State::Occupied => {
                        if count_taken((y, x), map) >= 4 {
                            lib::State::Empty
                        } else {
                            state
                        }
                    }
                })
                .collect()
        })
        .collect()
}

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let mut map = lib::read_map(stdin);

    loop {
        let new_map = new_map(&map);

        if map == new_map {
            break;
        } else {
            map = new_map;
        }
    }
    println!(
        "{}",
        map.iter()
            .flat_map(|line| line.iter().cloned())
            .filter(|&state| state == lib::State::Occupied)
            .count()
    )
}
