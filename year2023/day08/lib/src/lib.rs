use std::{collections::HashMap, fmt::Debug, str::FromStr};

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
#[repr(transparent)]
pub struct Node(u16);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Dir {
    L,
    R,
}

fn bits2char(v: u16) -> char {
    let b: u8 = (v & 0x1F) as u8;
    (b + b'A') as _
}

fn char2bits(c: char) -> u16 {
    let b: u8 = c.try_into().unwrap();

    b.checked_sub(b'A').unwrap() as _
}

impl Node {
    pub fn last_char(&self) -> char {
        bits2char(self.0 >> 10)
    }
}

impl Debug for Node {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chars = [
            bits2char(self.0),
            bits2char(self.0 >> 5),
            bits2char(self.0 >> 10),
        ];
        write!(f, "\"{}{}{}\"", chars[0], chars[1], chars[2])
    }
}

impl FromStr for Node {
    type Err = &'static str;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut acc = 0;
        for (i, c) in s.chars().enumerate() {
            acc |= char2bits(c) << (5 * i as u32);
        }
        Ok(Self(acc))
    }
}

impl Dir {
    #[inline]
    pub fn select<T>(self, pair: &(T, T)) -> &T {
        match self {
            Dir::L => &pair.0,
            Dir::R => &pair.1,
        }
    }
}

impl TryFrom<char> for Dir {
    type Error = &'static str;

    #[inline]
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Dir::L),
            'R' => Ok(Dir::R),
            _ => Err("unknown direction"),
        }
    }
}

pub fn read_scene() -> (Vec<Dir>, HashMap<Node, (Node, Node)>) {
    let mut lines = std::io::stdin().lines();
    let path_str = lines.next().unwrap().unwrap();
    lines.next().unwrap().unwrap(); // empty line

    let path = path_str
        .chars()
        .map(|c| Dir::try_from(c))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let mut labirinth = HashMap::new();
    for line in lines {
        let line = line.unwrap();
        let line = line.trim();

        let src = Node::from_str(&line[0..3]).unwrap();
        let left = Node::from_str(&line[7..10]).unwrap();
        let right = Node::from_str(&line[12..15]).unwrap();
        labirinth.insert(src, (left, right));
    }

    (path, labirinth)
}
