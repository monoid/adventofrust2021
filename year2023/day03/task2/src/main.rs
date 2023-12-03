use std::collections::{HashMap, HashSet};

fn main() {
    let scene = lib::read_scene();

    let size = (scene[0].len(), scene.len());

    let mut acc = 0;

    let mut all_nums = HashMap::new();

    for (y, line) in scene.iter().enumerate() {
        let mut it = line.as_str();
        let mut offset = 0;
        // working with bytes strings would be easier
        while let Some(start_x) = it.find(|c| ('0'..='9').contains(&c)) {
            // we've found a start of number; skip the garbage
            it = &it[start_x..];
            offset += start_x;

            let (num, rest) = match it.find(|c| !('0'..='9').contains(&c)) {
                Some(end_num) => it.split_at(end_num),
                None => (it, ""),
            };

            // handle it
            let val: u32 = num.parse().unwrap();
            let x1 = offset;
            let x2 = offset + num.len();

            for x in x1..x2 {
                all_nums.insert((x, y), (x1, y, val));
            }

            it = rest;
            offset = x2;
        }
    }

    for (y, line) in scene.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '*' {
                let mut nums = HashSet::new();

                for (x, y) in lib::around((x, y), size) {
                    if let Some(p) = all_nums.get(&(x, y)).cloned() {
                        nums.insert(p);
                    }
                }

                if nums.len() == 2 {
                    acc += nums.into_iter().map(|(_, _, val)| val).product::<u32>();
                }
            }
        }
    }

    println!("{}", acc);
}
