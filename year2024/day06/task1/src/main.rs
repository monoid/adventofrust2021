fn main() {
    let map = lib::Map::read(std::io::stdin().lock());
    let visited = lib::find_visited(&map);
    println!("{}", visited.len());
}
