use std::collections::HashMap;
use std::io::BufRead;
use std::str::FromStr;

pub type Stone = u64;

pub const MULT: Stone = 2024;

#[derive(Debug)]
pub struct Stones {
    stones: HashMap<Stone, usize>,
}

enum Blink {
    Replace(u64),
    Split(u64, u64),
}

impl Stones {
    pub fn read<R: BufRead>(mut inp: R) -> Self {
        let mut line = String::new();
        inp.read_to_string(&mut line).unwrap();
        let line = line.trim();
        let stones_inp: Result<Vec<Stone>, _> = line
            .split_ascii_whitespace()
            .map(FromStr::from_str)
            .collect();
        let mut stones = HashMap::new();
        for stone in stones_inp.unwrap() {
            *stones.entry(stone).or_default() += 1;
        }
        Self { stones }
    }

    pub fn blink(&mut self) {
        let old_stones = std::mem::take(&mut self.stones);
        for (stone, cnt) in old_stones {
            match blink(stone) {
                Blink::Replace(n) => *self.stones.entry(n).or_default() += cnt,
                Blink::Split(a, b) => {
                    *self.stones.entry(a).or_default() += cnt;
                    *self.stones.entry(b).or_default() += cnt;
                }
            }
        }
    }

    pub fn len(&self) -> usize {
        self.stones.values().sum()
    }
}

fn blink(val: Stone) -> Blink {
    if val == 0 {
        Blink::Replace(1)
    } else {
        let mut s = val.to_string();
        if s.len() % 2 == 0 {
            let sb = s.split_off(s.len() / 2);
            Blink::Split(sb.parse().unwrap(), s.parse().unwrap())
        } else {
            Blink::Replace(val * MULT)
        }
    }
}
