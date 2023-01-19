fn main() {
    let mut data = lib::Map::read();
    let val = data.eval2();
    println!("{}", val);
}
