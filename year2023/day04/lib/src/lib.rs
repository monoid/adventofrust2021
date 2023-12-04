use std::{collections::HashSet, str::FromStr};

pub struct Card {
    pub id: u32,
    pub scratched: HashSet<u8>,
    pub winners: HashSet<u8>,
}

impl Card {
    pub fn parse_card(inp: &str) -> Self {
        assert!(inp.is_ascii());

        let inp = inp
            .strip_prefix("Card ")
            .expect("malformed input line: No 'Card ' prefix");
        let (id_str, body_str) = inp
            .split_once(":")
            .expect("malformed input line: on ':' separator");
        let id = id_str.trim().parse().expect("malformed card ID");

        let (scratched_str, winners_str) = body_str
            .split_once("|")
            .expect("malformed input line: no bar delimeter");

        let scratched =
            parse_card_numbers(scratched_str.trim_end()).expect("malformed scratched field");
        let winners = parse_card_numbers(winners_str).expect("malformed winners field");

        Self {
            id,
            scratched,
            winners,
        }
    }

    #[inline]
    pub fn score_v1(&self) -> usize {
        let matches = self.matches();
        if matches == 0 {
            0
        } else {
            1 << (matches - 1)
        }
    }

    #[inline]
    pub fn matches(&self) -> usize {
        self.scratched.intersection(&self.winners).count()
    }
}

fn parse_card_numbers(nums: &str) -> Result<HashSet<u8>, std::num::ParseIntError> {
    nums.split_whitespace().map(u8::from_str).collect()
}

pub fn read_data() -> Vec<Card> {
    std::io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let line = line.trim();

            Card::parse_card(line)
        })
        .collect()
}
