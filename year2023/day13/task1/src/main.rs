fn main() {
    let data = lib::read_data();
    let mut hor = 0;
    let mut ver = 0;

    for m in data.iter().map(|m| lib::Map::find_reflection(m, 0)) {
        match m {
            lib::Axis::Vert(v) => ver += v,
            lib::Axis::Hor(h) => hor += h,
        }
    }
    eprintln!("{}", 100 * hor + ver);
}
