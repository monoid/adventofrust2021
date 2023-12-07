use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    High,
    One,
    Two,
    Three,
    Full,
    Four,
    Five,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HandV1(Kind, Cards);

impl HandV1 {
    pub fn parse(s: &str) -> Self {
        let cards_vec: Vec<u8> = s.chars().map(parse_card_v1).collect();
        assert_eq!(cards_vec.len(), 5); // I do not bother with error handling, sorry.

        let cards: Cards = cards_vec.try_into().unwrap();
        let kind = kind_v1(cards);

        Self(kind, cards)
    }
}

fn parse_card_v1(card: char) -> u8 {
    match card {
        '1'..='9' => (card as u8) - b'0',
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("unknown card type: {:?}", card),
    }
}

type Cards = [u8; 5];

fn kind_v1(cards: Cards) -> Kind {
    let mut counts = HashMap::<u8, u8>::new();
    for c in &cards[..] {
        *counts.entry(*c).or_default() += 1;
    }
    match counts.len() {
        1 => Kind::Five,
        2 => {
            let val = *counts.values().next().unwrap();
            if val == 2 || val == 3 {
                Kind::Full
            } else {
                Kind::Four
            }
        }
        3 => {
            let max_count = counts.values().cloned().max().unwrap();
            if max_count == 3 {
                // AAABC
                Kind::Three
            } else {
                Kind::Two
            }
        }
        4 => Kind::One,
        5 => Kind::High,
        _ => unreachable!("can't happen"),
    }
}

pub fn parse_line_v1(line: &str) -> (HandV1, u32) {
    let (card, val) = line.trim().split_once(' ').unwrap();
    (HandV1::parse(card), val.parse().unwrap())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HandV2(Kind, Cards);

impl HandV2 {
    pub fn parse(s: &str) -> Self {
        let cards_vec: Vec<u8> = s.chars().map(parse_card_v2).collect();
        assert_eq!(cards_vec.len(), 5); // I do not bother with error handling, sorry.

        let cards: Cards = cards_vec.try_into().unwrap();
        let kind = kind_v2(cards);

        Self(kind, cards)
    }
}

fn parse_card_v2(card: char) -> u8 {
    match card {
        'J' => 0,
        '1'..='9' => (card as u8) - b'0',
        'T' => 10,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("unknown card type: {:?}", card),
    }
}

fn kind_v2(cards: Cards) -> Kind {
    let mut uniq_cards = HashSet::<u8>::new();

    for c in &cards[..] {
        uniq_cards.insert(*c);
    }

    if uniq_cards.contains(&0) {
        uniq_cards
            .into_iter()
            .map(|jok| {
                // replace the joker with alternative (including itself).
                // we may use any other card, but it doesn't seem to improve kind.
                let new_cards: Cards = cards[..]
                    .iter()
                    .cloned()
                    .map(|c| if c == 0 { jok } else { c })
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                kind_v1(new_cards)
            })
            .max()
            .unwrap()
    } else {
        kind_v1(cards)
    }
}

pub fn parse_line_v2(line: &str) -> (HandV2, u32) {
    let (card, val) = line.trim().split_once(' ').unwrap();
    (HandV2::parse(card), val.parse().unwrap())
}
