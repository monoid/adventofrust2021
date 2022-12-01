use std::io::{self, BufRead};

pub fn lines() -> impl Iterator<Item = Result<String, io::Error>> {
    let input = io::stdin();
    input.lock().lines()
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Elf {
    items: Vec<u32>,
}

impl Elf {
    pub fn calories(&self) -> u32 {
        self.items.iter().sum()
    }
}

pub fn elves<I: Iterator<Item = impl Into<String>>>(lines: I) -> Vec<Elf> {
    let mut current_items = Elf::default();
    let mut elves = vec![];
    for line in lines {
        let line = line.into();
        if line.is_empty() {
            let mut items = Elf::default();
            std::mem::swap(&mut current_items, &mut items);
            elves.push(items);
        } else {
            current_items.items.push(line.parse().unwrap());
        }
    }
    if !current_items.items.is_empty() {
        elves.push(current_items);
    }
    elves
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_items() {
        let items = elves(vec!["100", "200", "", "300"].into_iter());
        assert_eq!(
            items,
            vec![
                Elf {
                    items: vec![100, 200]
                },
                Elf { items: vec![300] },
            ]
        )
    }
}
