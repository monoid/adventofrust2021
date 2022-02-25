use std::{collections::HashMap, io};

fn main() {
    let data = lib::read_data(io::stdin().lock()).unwrap();
    let mut tiles = HashMap::default();
    for line in data {
        lib::apply_tile(&mut tiles, &line);
    }
    println!("{}", tiles.values().filter(|&&v| v).count());
}
