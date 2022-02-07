fn main() {
    let mut seats = lib::seats();
    seats.sort_unstable();
    let w = seats.windows(2).find(|w| w[0] + 2 == w[1]).unwrap();
    println!("{}", w[0] + 1);
}
