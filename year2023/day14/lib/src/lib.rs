pub fn read_data() -> Vec<Vec<u8>> {
    std::io::stdin()
        .lines()
        .map(|l| l.unwrap().into())
        .collect()
}

pub fn tilt_north(data: &mut [Vec<u8>]) {
    let width = data[0].len();
    let height = data.len();

    for x in 0..width {
        let mut dst = 0;

        for y in 0..height {
            match data[y][x] {
                b'#' => {
                    dst = y + 1;
                }
                b'.' => {
                    // Do nothing.
                }
                b'O' => {
                    data[y][x] = b'.';
                    data[dst][x] = b'O';
                    dst += 1;
                }
                unknown => panic!("unknown byte: {}", unknown),
            }
        }
    }
}

pub fn tilt_south(data: &mut [Vec<u8>]) {
    let width = data[0].len();
    let height = data.len();

    for x in 0..width {
        let mut dst = height - 1;

        for y in (0..height).rev() {
            match data[y][x] {
                b'#' => {
                    dst = y.saturating_sub(1);
                }
                b'.' => {
                    // Do nothing.
                }
                b'O' => {
                    data[y][x] = b'.';
                    data[dst][x] = b'O';
                    dst = dst.saturating_sub(1);
                }
                unknown => panic!("unknown byte: {}", unknown),
            }
        }
    }
}

pub fn tilt_west(map: &mut [Vec<u8>]) {
    let width = map[0].len();
    let height = map.len();

    #[allow(clippy::needless_range_loop)]
    for y in 0..height {
        let mut dst = 0;

        for x in 0..width {
            match map[y][x] {
                b'#' => {
                    dst = x + 1;
                }
                b'O' => {
                    map[y][x] = b'.';
                    map[y][dst] = b'O';
                    dst += 1;
                }
                b'.' => {}
                other => panic!("unknown byte {other:?}"),
            }
        }
    }
}

pub fn tilt_east(map: &mut [Vec<u8>]) {
    let width = map[0].len();
    let height = map.len();

    #[allow(clippy::needless_range_loop)]
    for y in 0..height {
        let mut dst = width - 1;

        for x in (0..width).rev() {
            match map[y][x] {
                b'#' => {
                    dst = x.saturating_sub(1);
                }
                b'O' => {
                    map[y][x] = b'.';
                    map[y][dst] = b'O';
                    dst = dst.saturating_sub(1);
                }
                b'.' => {}
                other => panic!("unknown byte {other:?}"),
            }
        }
    }
}

pub fn calc_load(data: &[Vec<u8>]) -> usize {
    data.iter()
        .rev()
        .enumerate()
        .map(|(i, line)| (i + 1) * line.iter().filter(|&&c| c == b'O').count())
        .sum()
}
