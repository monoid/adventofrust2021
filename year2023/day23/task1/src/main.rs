fn main() {
    let map = lib::Map::read_data();
    let len = map.find_longest_path_len();
    eprintln!("{len}");
}
