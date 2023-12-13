#[derive(Debug)]
pub struct Line {
    pub chars: Vec<u8>,
    pub groups: Vec<usize>,
}

impl Line {
    pub fn parse_line(line: &str) -> Self {
        let (chars, groups) = line.trim().split_once(' ').unwrap();
        let groups: Vec<usize> = groups.split(',').map(|n| n.parse().unwrap()).collect();

        Self {
            chars: chars.as_bytes().to_owned(),
            groups,
        }
    }

    pub fn count_possibilities(&self) -> u64 {
        // the table:
        //    vec[group_num][offset]: how many valid combinations can exists if (group_num + 1) last groups
        //                            are placed starting from the offset (i.e. if the first placed group
        //                            is located at offset, and other are further).
        let mut state: Vec<Vec<u64>> = vec![vec![0; self.chars.len()]; self.groups.len()];

        // Start filing the table.
        let last_group_len = *self.groups.last().unwrap();
        for (pos, cell) in state[0].iter_mut().enumerate() {
            if pos + last_group_len > self.chars.len() {
                // cannot span that much
                *cell = 0;
            } else if can_be_placed_at(last_group_len, &self.chars[pos..pos + last_group_len])
                && can_be_free(&self.chars[pos + last_group_len..])
            {
                *cell = 1;
            } else {
                *cell = 0;
            }
        }

        for (idx, group_len) in self.groups.iter().rev().enumerate().skip(1) {
            assert!(idx > 0);
            for pos in 0..self.chars.len() {
                if pos + group_len > self.chars.len() {
                    // cannot span that much
                    state[idx][pos] = 0;
                } else {
                    let mut cnt = 0;
                    for next_pos in pos + group_len + 1..self.chars.len() {
                        if can_be_placed_at(*group_len, &self.chars[pos..pos + group_len])
                            && can_be_free(&self.chars[pos + group_len..next_pos])
                        {
                            cnt += state[idx - 1][next_pos];
                        }
                    }
                    state[idx][pos] = cnt;
                }
            }
        }

        state
            .last()
            .unwrap()
            .iter()
            .enumerate()
            .map(|(pos, cnt)| cnt * (can_be_free(&self.chars[..pos]) as u64))
            .sum()
    }
}

fn can_be_placed_at(group_len: usize, part: &[u8]) -> bool {
    part[..group_len].iter().all(|c| *c != b'.')
}

fn can_be_free(part: &[u8]) -> bool {
    part.iter().all(|c| *c != b'#')
}

pub fn read_data() -> Vec<Line> {
    std::io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            Line::parse_line(&line)
        })
        .collect()
}

pub fn read_data5x() -> Vec<Line> {
    std::io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (a, b) = line.trim().split_once(' ').unwrap();
            Line::parse_line(&format!("{a}?{a}?{a}?{a}?{a} {b},{b},{b},{b},{b}"))
        })
        .collect()
}
