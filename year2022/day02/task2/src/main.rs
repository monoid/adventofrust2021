fn main() {
    let script = lib::read_script::<lib::Outcome>();
    let score = script
        .iter()
        .cloned()
        .map(|(opp, out)| lib::score(opp, lib::move_by_outcome(opp, out)))
        .sum::<i32>();
    println!("{}", score);
}
