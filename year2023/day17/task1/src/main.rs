fn main() {
    let data = lib::Map::read_data();
    let w = data.find_best_path_to_corner(1, 3);
    println!("{w}");
}
