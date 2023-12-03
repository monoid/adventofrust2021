pub fn read_scene() -> Vec<String> {
    std::io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let line = line.trim();
            line.to_string()
        })
        .collect()
}

pub fn around(pos: (usize, usize), size: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    use itertools::Itertools;

    let pos = (pos.0 as isize, pos.1 as isize);
    let size = (size.0 as isize, size.1 as isize);

    (-1isize..=1)
        .cartesian_product(-1isize..=1)
        .filter_map(move |(dx, dy)| {
            if (dx == 0) && (dy == 0) {
                // Always skip the center
                None
            } else {
                let nx = pos.0 + dx;
                let ny = pos.1 + dy;

                if (nx >= 0) && (ny >= 0) && (nx < size.0) && (ny < size.1) {
                    Some((nx as usize, ny as usize))
                } else {
                    None
                }
            }
        })
}
