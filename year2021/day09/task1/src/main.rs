use std::io::{self, BufRead as _};

fn find_low_points<'a, T: AsRef<[i8]> + 'a>(
    v: &'a [T],
    width: usize,
) -> impl Iterator<Item = (usize, usize, i8)> + 'a {
    let height = v.len();

    (0..height)
        .flat_map(move |h| (0..width).map(move |w| (h, w)))
        .filter_map(move |(h, w)| {
            let c = v[h].as_ref()[w];
            let is_low = [(-1isize, 0isize), (1, 0), (0, -1), (0, 1)]
                .into_iter()
                .all(|(dx, dy)| {
                    let h1 = (h as isize) + dy;
                    let w1 = (w as isize) + dx;
                    (h1 < 0)
                        || (h1 >= (height as isize))
                        || (w1 < 0)
                        || (w1 >= (width as isize))
                        || (v[h1 as usize].as_ref()[w1 as usize] > c)
                });
            if is_low {
                Some((h, w, c))
            } else {
                None
            }
        })
}

fn main() {
    let map: Vec<Vec<i8>> = io::stdin()
        .lock()
        .lines()
        .map(|lin| {
            lin.unwrap()
                .chars()
                .map(|c| c as i8 - ('0' as i8))
                .collect()
        })
        .collect();

    println!(
        "{}",
        find_low_points(&map, map[0].len())
            .map(|(_, _, c)| (c as u32) + 1)
            .sum::<u32>()
    );
}
