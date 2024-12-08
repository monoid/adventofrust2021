use std::str::FromStr;

fn main() {
    let rules: Vec<_> = std::io::stdin().lines().map(|r| {
        let line = r.unwrap();
        lib::Rule::from_str(&line).unwrap_or_else(|_| panic!("{line}"))
    }).collect();
    let mut sum = 0;

    for rule in rules {
        if rule.solvable() {
            sum += rule.target;
        }
    }

    println!("{sum}");
}
