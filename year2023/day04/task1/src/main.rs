fn main() {
    let data = lib::read_data();

    let score = data.iter().map(lib::Card::score_v1).sum::<usize>();
    println!("{}", score);
}
