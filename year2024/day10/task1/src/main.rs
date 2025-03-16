fn main() {
    let map = lib::Map::read(std::io::stdin().lock());
    let solution = map.solve1();
    println!("{solution}");
}
