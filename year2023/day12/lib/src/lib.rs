use std::ops::Range;

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
        // We should iterate only over possible positions (i.e. only positions with '#' and '?')
        // Or not, as it doens't seem to improve a lot.
        let mut pos_buf = Vec::<State>::with_capacity(self.groups.len());
        let mut counter = 0u64;

        // The init loop
        loop {
            while pos_buf.len() < self.groups.len() {
                // loop over what?
                let next_available_pos = pos_buf
                    .last()
                    .map(|state| state.pos + state.group_len + 1)
                    .unwrap_or(0);

                let current_group = pos_buf.len();
                let state = State {
                    pos: next_available_pos as _,
                    group_len: self.groups[current_group] as _,
                    possible: next_available_pos + 1..(self.chars.len() as u8),
                };
                match state.place(&self.chars) {
                    Some(placed_state) => {
                        pos_buf.push(placed_state);
                    }
                    None => break, // rollback
                }
            }
            if pos_buf.len() == self.groups.len() && in_constraints(&pos_buf, &self.chars) {
                counter += 1;
            }

            while !pos_buf.is_empty() {
                let top_state = pos_buf.pop().expect("TODO");
                if let Some(new_top_state) = top_state.advance() {
                    if let Some(placed_top_state) = new_top_state.place(&self.chars) {
                        pos_buf.push(placed_top_state);
                        break;
                    }
                }
            }
            if pos_buf.is_empty() {
                break;
            }
        }

        counter
    }
}

fn in_constraints(groups: &[State], chars: &[u8]) -> bool {
    assert!(!groups.is_empty());
    if !chars[0..groups[0].pos as usize]
        .iter()
        .cloned()
        .all(|b| b != b'#')
    {
        return false;
    }
    let last = groups.last().unwrap();
    if !chars[(last.pos + last.group_len) as usize..]
        .iter()
        .cloned()
        .all(|b| b != b'#')
    {
        return false;
    }
    for idx in 1..groups.len() {
        let p0 = groups[idx - 1].pos + groups[idx - 1].group_len;
        let p1 = groups[idx].pos;

        if !chars[p0 as usize..p1 as usize]
            .iter()
            .cloned()
            .all(|b| b != b'#')
        {
            return false;
        }
    }
    true
}

#[derive(Debug, Clone)]
struct State {
    pos: u8,
    group_len: u8,
    possible: Range<u8>,
}

impl State {
    fn matches(&self, line: &[u8]) -> bool {
        if self.pos + self.group_len > line.len() as u8 {
            return false;
        }
        line[self.pos as usize..(self.pos + self.group_len) as usize]
            .iter()
            .cloned()
            .all(|b| b != b'.')
    }

    fn advance(mut self) -> Option<Self> {
        self.possible.next().map(|next_pos| {
            self.pos = next_pos;
            self
        })
    }

    fn place(self, line: &[u8]) -> Option<Self> {
        let mut self_ = self;
        if self_.matches(line) {
            return Some(self_);
        }
        loop {
            match self_.advance() {
                Some(new_self) => {
                    if new_self.matches(line) {
                        return Some(new_self);
                    } else {
                        self_ = new_self;
                    }
                }
                None => break,
            }
        }
        None
    }
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
            Line::parse_line(&format!("{a}{a}{a}{a}{a} {b},{b},{b},{b},{b}"))
        })
        .collect()
}
