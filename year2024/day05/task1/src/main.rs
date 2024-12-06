fn main() {
    let data = lib::Data::read(std::io::stdin());
    let mut sum = 0;
    'outer: for man in &data.manuals {
        for rule in &data.rules {
            let before = man.iter().position(|n| n == &rule.before);
            let after = man.iter().position(|n| n == &rule.after);
            if !before
                .zip(after)
                .map(|(before_idx, after_idx)| before_idx < after_idx)
                .unwrap_or(true)
            {
                continue 'outer;
            }
        }
        sum += man[man.len() / 2];
    }
    println!("{sum}");
}
