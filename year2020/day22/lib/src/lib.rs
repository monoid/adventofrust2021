use std::{collections::VecDeque, io};

pub enum Winner {
    Player1,
    Player2,
}

pub fn score(deck: &VecDeque<u8>) -> usize {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(i, &v)| -> usize { (i + 1) * (v as usize) })
        .sum()
}

pub fn read_data<R: io::BufRead>(inp: R) -> (VecDeque<u8>, VecDeque<u8>) {
    let mut d1: VecDeque<u8> = Default::default();
    let mut d2: VecDeque<u8> = Default::default();

    let mut lines = inp.lines();
    let l = lines.next().unwrap().unwrap();
    assert!(l.starts_with("Player "));

    loop {
        let l = lines.next().unwrap().unwrap();
        if l.is_empty() {
            break;
        } else {
            d1.push_back(l.parse().unwrap());
        }
    }

    let l = lines.next().unwrap().unwrap();
    assert!(l.starts_with("Player "));

    for lr in lines {
        let l = lr.unwrap();
        d2.push_back(l.parse().unwrap());
    }

    (d1, d2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
