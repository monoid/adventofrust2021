use std::io;

fn main() {
    let (a, b) = lib::read_data(io::stdin().lock());

    let r = lib::find_key(a);
    println!("{}", lib::apply_rounds(b, r));
}
