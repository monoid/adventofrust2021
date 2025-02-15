fn main() {
    let map = lib::Map::read_map(std::io::stdin().lock());
    let cnt = map.count_spots2();
    println!("{}", cnt);
}
