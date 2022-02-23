use std::{
    collections::{HashSet, VecDeque},
    io,
};

fn game(mut d1: VecDeque<u8>, mut d2: VecDeque<u8>) -> (lib::Winner, usize) {
    let mut history = HashSet::<(Box<[u8]>, Box<[u8]>)>::new();
    while !d1.is_empty() && !d2.is_empty() {
        d1.make_contiguous();
        d2.make_contiguous();

        let uniq = (d1.as_slices().0.into(), d2.as_slices().0.into());
        if !history.insert(uniq) {
            return (lib::Winner::Player1, lib::score(&d1));
        }

        let v1 = d1.pop_front().unwrap().into();
        let v2 = d2.pop_front().unwrap().into();
        let winner = if d1.len() >= v1 && d2.len() >= v2 {
            assert!(d1.as_slices().1.is_empty());
            assert!(d2.as_slices().1.is_empty());
            game(
                d1.as_slices().0[..v1].iter().cloned().collect(),
                d2.as_slices().0[..v2].iter().cloned().collect(),
            )
            .0
        } else if v1 > v2 {
            lib::Winner::Player1
        } else {
            lib::Winner::Player2
        };
        match winner {
            lib::Winner::Player1 => {
                d1.push_back(v1 as _);
                d1.push_back(v2 as _);
            }
            lib::Winner::Player2 => {
                d2.push_back(v2 as _);
                d2.push_back(v1 as _);
            }
        };
    }
    if d1.is_empty() {
        (lib::Winner::Player2, lib::score(&d2))
    } else {
        (lib::Winner::Player1, lib::score(&d1))
    }
}

fn main() {
    let (d1, d2) = lib::read_data(io::stdin().lock());

    println!("{}", game(d1, d2).1);
}
