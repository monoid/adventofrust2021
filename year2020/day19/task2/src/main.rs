use std::io;

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let mut rules = lib::read_rules(&mut stdin).unwrap();
    rules.insert(8, lib::Rule::Rules(vec![
        vec![42, 8],
        vec![42],
    ]));
    rules.insert(11, lib::Rule::Rules(vec![
        vec![42, 11, 31],
        vec![42, 31],
    ]));
    // This may work only for particular data.
    rules.insert(0, lib::Rule::Nd(8, 11));
    println!("{}", lib::count_matches(&mut stdin, &rules));
}
