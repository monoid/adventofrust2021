fn main() {
    let mut map = lib::Map::read2();
    let val = map.search();
    println!("{}", val);
}
