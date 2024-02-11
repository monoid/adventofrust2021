fn main() {
    let mut map = lib::Map::read_data();
    map.slippery = false;
    let len = map.find_longest_path_len();
    eprintln!("{len}");
}
