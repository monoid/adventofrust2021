fn main() {
    let map = lib::read_map();
    println!("{}", lib::max_scenic_score(&map).unwrap());
}
