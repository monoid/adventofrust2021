use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
}

type TileId = (i32, i32);

// TODO it has to be a compressed representation.  How?
// Rectangles? Ranges?
type Tiles = HashSet<TileId>;
