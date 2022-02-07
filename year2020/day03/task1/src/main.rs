fn main() {
    let map = lib::read_map().unwrap();
    let dx = 3;
    let dy = 1;

    println!("{}", lib::count_trees(&map, dx, dy));
}
