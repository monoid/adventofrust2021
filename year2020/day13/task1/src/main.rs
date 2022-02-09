use std::io;

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let (ts, data) = lib::parse_input(&mut stdin);

    let res = data
        .iter()
        .cloned()
        .flatten()
        .map(|n| (n - (ts % n), n))
        .min_by_key(|&(w, _)| w);
    println!("{}", res.unwrap().1 * res.unwrap().0);
}
