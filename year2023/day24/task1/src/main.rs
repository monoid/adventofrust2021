fn main() {
    let data = lib::read_data();
    for pt in data {
        // no velocity is horizontal or vertical
        if pt.velocity.0 .0 .0 == 0 || pt.velocity.0 .0 .1 == 0 {
            eprintln!("{pt:?}");
        }
    }
}
