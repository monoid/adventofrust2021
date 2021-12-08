use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    io::{self, BufRead as _},
};

/*
 * | a | 8 |
 * | b | 6 |
 * | c | 8 |
 * | d | 7 |
 * | e | 4 |
 * | f | 9 |
 * | g | 7 |

 * | e | 4 |
 * | b | 6 |
 * | d | 7 |
 * | g | 7 |
 * | a | 8 | 7
 * | c | 8 | 1,7
 * | f | 9 | 1,7

 g appears everywhere except 1, 4, 7

 a = 7\1
 b: 6 times
 c: in 1, 8 times
 e: 4 times
 f: in 1, 9 times
 g: everywhere except 1, 4, 7
 d: the remaining one
*/

fn common_chars<'a>(a: Cow<'a, str>, b: Cow<'a, str>) -> Cow<'a, str> {
    let a: Vec<_> = a.chars().collect();
    b.chars()
        .filter(|c| a.contains(c))
        .collect::<String>()
        .into()
}

fn convert_to_digits(samples: &str, digits: &str) -> u32 {
    // Distribution of chars
    let mut char_freqs = HashMap::<char, u8>::new();
    for c in samples.chars() {
        if c.is_alphabetic() {
            *char_freqs.entry(c).or_default() += 1;
        }
    }
    assert_eq!(char_freqs.values().sum::<u8>(), 49);

    let mut v: [char; 7] = [' ', ' ', ' ', ' ', ' ', ' ', ' '];

    let seven = samples
        .split_ascii_whitespace()
        .find(|s| s.len() == 3)
        .unwrap();
    let one = samples
        .split_ascii_whitespace()
        .find(|s| s.len() == 2)
        .unwrap();
    let the_a = seven.chars().find(|c| one.find(*c).is_none()).unwrap();
    v[0] = the_a;

    let the_b = *char_freqs.iter().find(|(_, f)| **f == 6).unwrap().0;
    v[1] = the_b;

    let the_c = one.chars().find(|c| char_freqs.get(c) == Some(&8)).unwrap();
    v[2] = the_c;

    // v[3] = the_d is the last one

    let the_e = *char_freqs.iter().find(|(_, f)| **f == 4).unwrap().0;
    v[4] = the_e;

    let the_f = one.chars().find(|c| char_freqs.get(c) == Some(&9)).unwrap();
    v[5] = the_f;

    let the_g = samples
        .split_ascii_whitespace()
        .filter(|s| s.len() > 4)
        .map(Cow::from)
        .reduce(common_chars)
        .unwrap()
        .chars()
        .find(|&c| c != the_a)
        .unwrap();
    v[6] = the_g;

    let the_d = ('a'..='g').filter(|c| !v[..].contains(c)).next().unwrap();
    v[3] = the_d;

    assert_eq!(HashSet::<char>::from_iter(v.iter().cloned()).len(), 7);
    //
    let mut dig_array: Vec<Vec<char>> = vec![
        Vec::from_iter([v[0], v[1], v[2], v[4], v[5], v[6]].into_iter()), // 0
        Vec::from_iter([v[2], v[5]].into_iter()),                         // 1
        Vec::from_iter([v[0], v[2], v[3], v[4], v[6]].into_iter()),       // 2
        Vec::from_iter([v[0], v[2], v[3], v[5], v[6]].into_iter()),       // 3
        Vec::from_iter([v[1], v[2], v[3], v[5]].into_iter()),             // 4
        Vec::from_iter([v[0], v[1], v[3], v[5], v[6]].into_iter()),       // 5
        Vec::from_iter([v[0], v[1], v[3], v[4], v[5], v[6]].into_iter()), // 6
        Vec::from_iter([v[0], v[2], v[5]].into_iter()),                   // 7
        Vec::from_iter(v.iter().cloned()),                                // 8,
        Vec::from_iter([v[0], v[1], v[2], v[3], v[5], v[6]].into_iter()), // 9
    ];
    dig_array.iter_mut().for_each(|a| a.sort());

    let dig_res: Vec<_> = digits
        .split_ascii_whitespace()
        .map(|s| {
            let mut v: Vec<_> = s.chars().collect();
            v.sort();
            dig_array.iter().position(|d| d == &v).unwrap()
        })
        .collect();
    dig_res.into_iter().fold(0, |a, b| a * 10 + (b as u32))
}

fn main() {
    let mut sum = 0;
    for line in io::stdin().lock().lines() {
        if let Some((samples, digits)) = line.unwrap().split_once('|') {
            sum += convert_to_digits(samples.trim(), digits.trim());
        }
    }
    println!("{}", sum);
}
