fn main() {
    let script = lib::read_script::<lib::Move>();
    let score = script
        .iter()
        .cloned()
        .map(|(opp, our)| lib::score(opp, our))
        .sum::<i32>();
    println!("{}", score);
}
