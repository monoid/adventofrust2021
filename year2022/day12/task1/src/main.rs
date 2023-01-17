fn main() {
    let mut map = lib::Map::read1();
    let val = map.search();
    println!("{}", val);
}
