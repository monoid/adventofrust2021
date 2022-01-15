use std::collections::HashSet;

const YEAR: i32 = 2020;

fn main() {
    let data = lib::read_input();

    let mut found = HashSet::new();

    for n in data {
        let opp = YEAR - n;
        if found.contains(&opp) {
            println!("{}", n * opp);
            break;
        }
        found.insert(n);
    }
}
