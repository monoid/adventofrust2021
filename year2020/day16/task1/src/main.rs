use std::io;

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let (rules, _, tickets) = lib::read_input(stdin);
    let invalid = tickets
        .iter()
        .map(|tick| {
            tick.iter()
                .filter(|&&val| !rules.iter().any(|rule| rule.validate1(val)))
                .sum::<u32>()
        })
        .sum::<u32>();
    println!("{}", invalid);
}
