fn main() {
    let map = lib::read_data();
    let count = map.iterate(64);
    eprintln!("{count}");
}
