#[derive(Debug)]
pub struct Map {
    pub data: Vec<String>,
}

#[derive(Debug, Copy, Clone)]
pub enum Axis {
    Vert(usize),
    Hor(usize),
}

impl Map {
    pub fn find_reflection(&self, diff: usize) -> Axis {
        // Try horizontal.
        for possible in 1..self.data.len() {
            let len_up = possible;
            let len_lo = self.data.len() - possible;
            let len_mir = std::cmp::min(len_up, len_lo);

            let upper_part = &self.data[possible - len_mir..possible];
            let lower_part = &self.data[possible..possible + len_mir];

            if upper_part
                .iter()
                .rev()
                .zip(lower_part)
                .map(|(a, b)| a.chars().zip(b.chars()).filter(|(a, b)| a != b).count())
                .sum::<usize>()
                == diff
            {
                return Axis::Hor(possible);
            }
        }

        // If not, vertical.
        let width = self.data[0].len();
        for possible in 1..width {
            let len_left = possible;
            let len_righ = width - possible;
            let len_mir = std::cmp::min(len_left, len_righ);

            if (possible - len_mir..possible)
                .rev()
                .zip(possible..possible + len_mir)
                .map(|(xa, xb)| {
                    (0..self.data.len())
                        .filter(|y| self.data[*y].as_bytes()[xa] != self.data[*y].as_bytes()[xb])
                        .count()
                })
                .sum::<usize>()
                == diff
            {
                return Axis::Vert(possible);
            }
        }

        panic!("nothing found, invalid data: {:#?}", self)
    }
}

pub fn read_data() -> Vec<Map> {
    let lines = std::io::stdin().lines().map(|r| r.unwrap());
    let mut maps = vec![];

    let mut partial = vec![];

    for line in lines {
        if line.is_empty() {
            maps.push(Map {
                data: std::mem::take(&mut partial),
            });
        } else {
            partial.push(line);
        }
    }
    if !partial.is_empty() {
        maps.push(Map { data: partial });
    }

    maps
}
