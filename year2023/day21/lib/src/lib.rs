#[derive(Debug)]
pub struct Map {
    pub walls: Vec<Vec<W>>,
    pub start_pos: (usize, usize),
}

impl Map {
    pub fn iterate(&self, n: usize) -> u32 {
        let h = self.walls.len();
        let w = self.walls[0].len();
        let empty_map = vec![vec![0 as W; w]; h];
        let mut state_map = empty_map.clone();
        state_map[self.start_pos.1][self.start_pos.0 / (W::BITS as usize)] =
            1 << ((self.start_pos.0 as u32) % W::BITS);

        for _ in 0..n {
            let mut target = empty_map.clone();
            update_state(&mut target, &state_map, &self.walls);
            state_map = target; // TODO swap and clear
        }

        state_map
            .iter()
            .map(|row| row.iter().map(|&v| v.count_ones()).sum::<u32>())
            .sum()
    }
}

fn update_state(target: &mut [Vec<W>], prev: &[Vec<W>], walls: &[Vec<W>]) {
    for y in 0..target.len() {
        combine_left_right(&prev[y], &mut target[y]);
        if y == 0 {
            combine_vert(&prev[1], &mut target[0])
        } else if y + 1 == target.len() {
            combine_vert(&prev[y - 1], &mut target[y])
        } else {
            combine_vert2(&prev[y - 1], &prev[y + 1], &mut target[y]);
        }

        combine_walls(&walls[y], &mut target[y]);
    }
}

type W = u32;

// 3210 7654
//
// right 6543 (a >> 3) | (b << 1)
// left 4321  (a >> 1) | (b << 3)

fn combine_bytes(a: W, b: W, mut shift: u32) -> W {
    shift = shift & (W::BITS - 1);
    (a >> shift) | (b << (W::BITS - shift))
}

fn combine_left_right(src: &[W], target: &mut [W]) {
    // shift right
    let iter1 = src
        .iter()
        .cloned()
        .zip(&src[1..])
        .map(|(a, &b)| combine_bytes(a, b, W::BITS - 1));
    for (mask, cell) in iter1.zip(&mut target[1..]) {
        *cell |= mask;
    }
    // shift right need special handling of first element
    target[0] |= src[0] << 1;

    // shift left
    let iter2 = src
        .iter()
        .cloned()
        .zip(&src[1..])
        .map(|(a, &b)| combine_bytes(a, b, 1));
    for (mask, cell) in iter2.zip(&mut *target) {
        *cell |= mask;
    }
    // shift left need special handling of last element
    target[target.len() - 1] |= src[target.len() - 1] >> 1;
}

fn combine_vert(src: &[W], target: &mut [W]) {
    for (&mask, cell) in src.iter().zip(target) {
        *cell |= mask;
    }
}

fn combine_vert2(src1: &[W], src2: &[W], target: &mut [W]) {
    for ((&mask1, &mask2), cell) in src1.iter().zip(src2).zip(target) {
        *cell |= mask1 | mask2;
    }
}

fn combine_walls(walls: &[W], target: &mut [W]) {
    for (&wall_mask, cell) in walls.iter().zip(target) {
        *cell &= !wall_mask;
    }
}

pub fn read_data() -> Map {
    let lines = std::io::stdin().lines();
    let mut start_pos = None;
    let walls = lines
        .enumerate()
        .map(|(y, line)| {
            let line = line.unwrap();
            line.as_bytes()
                .chunks(W::BITS as usize)
                .enumerate()
                .map(|(chunk_num, chunk)| {
                    chunk
                        .iter()
                        .cloned()
                        .chain(std::iter::repeat(b'#'))
                        .take(W::BITS as usize)
                        .enumerate()
                        .fold(0 as W, |prev, (off, byte)| {
                            if byte == b'S' {
                                start_pos = Some((chunk_num * (W::BITS as usize) + off, y))
                            }
                            prev | (((byte == b'#') as W) << (off as u32))
                        })
                })
                .collect::<Vec<W>>()
        })
        .collect();
    Map {
        walls,
        start_pos: start_pos.unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_left_right_right() {
        let src = [0b10001000, 0];
        let mut target = [0, 0];
        combine_left_right(&src, &mut target);
        assert_eq!(target, [0b01010100, 0b1]);
    }

    #[test]
    fn test_left_right_left() {
        let src = [0, 0b00100011];
        let mut target = [0, 0];
        combine_left_right(&src, &mut target);
        assert_eq!(target, [0b10000000, 0b01010111]);
    }

    #[test]
    fn test_left_right_right_3() {
        let src = [0b10000000, 0, 0];
        let mut target = [0, 0, 0];
        combine_left_right(&src, &mut target);
        assert_eq!(target, [0b01000000, 0b1, 0]);
    }

    #[test]
    fn test_left_right_left_3() {
        let src = [0, 0, 1];
        let mut target = [0, 0, 0];
        combine_left_right(&src[..], &mut target[..]);
        assert_eq!(target, [0, 0b10000000, 0b10]);
    }

    #[test]
    fn test_left_right_both_3() {
        let src = [0b10000000, 0, 1];
        let mut target = [0, 0, 0];
        combine_left_right(&src, &mut target);
        assert_eq!(target, [0b01000000, 0b10000001, 0b10]);
    }
}
