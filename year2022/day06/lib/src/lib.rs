use std::borrow::BorrowMut;

#[derive(Default, Copy, Clone)]
struct LetterSet(u32);

impl LetterSet {
    fn size(self) -> u32 {
        self.0.count_ones()
    }

    fn xor(self, other: LetterSet) -> LetterSet {
        LetterSet(self.0 ^ other.0)
    }
}

impl TryFrom<char> for LetterSet {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        const BASE: u32 = b'a' as u32;

        match value {
            'a'..='z' => Ok(LetterSet(1 << ((value as u32) - BASE))),
            other => Err(other),
        }
    }
}

pub fn marker_position(inp: &str, len: u32) -> Option<usize> {
    let mut chars_to_add = inp.chars().enumerate();
    let mut letters = LetterSet::default();
    for (_, ch) in chars_to_add.borrow_mut().take((len - 1) as _) {
        letters = letters.xor(LetterSet::try_from(ch).unwrap());
    }
    let chars_to_remove = inp.chars();

    for ((idx, to_add), to_remove) in chars_to_add.zip(chars_to_remove) {
        letters = letters.xor(LetterSet::try_from(to_add).unwrap());
        if letters.size() == len {
            return Some(idx + 1);
        }

        letters = letters.xor(LetterSet::try_from(to_remove).unwrap());
    }
    None
}
