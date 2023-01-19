fn main() {
    let mut data = lib::Map::read();
    let val = data.eval1();
    println!("{}", val);
}
