use std::{collections::HashSet, io};

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let (rules, my_ticket, tickets) = lib::read_input(stdin);
    let mut names: Vec<Vec<HashSet<&str>>> = vec![vec![]; tickets[0].len()];
    for tick in tickets {
        if tick
            .iter()
            .all(|&val| rules.iter().any(|rule| rule.validate1(val)))
        {
            for (&val, name_cell) in tick.iter().zip(names.iter_mut()) {
                let possibles = HashSet::<_, _>::from_iter(rules.iter().filter_map(|rule| {
                    if rule.validate1(val) {
                        Some(rule.name.as_str())
                    } else {
                        None
                    }
                }));
                name_cell.push(possibles);
            }
        }
    }

    let mut names: Vec<_> = names
        .into_iter()
        .map(|sets| {
            sets.into_iter()
                .reduce(|a: HashSet<_>, b| a.intersection(&b).cloned().collect())
                .unwrap()
        })
        .enumerate()
        .collect();

    // The sets are not always one-element; however,
    // some sets are one-element; they are the only possible solutions,
    // and we preform something like a diagonal matrix process.
    names.sort_unstable_by_key(|(_, v)| HashSet::len(v));
    assert_eq!(names[0].1.len(), 1);
    for i in 1..names.len() {
        assert!(names[i - 1].1.len() == 1);
        let name = *names[i - 1].1.iter().next().unwrap();
        for m in &mut names[i..] {
            m.1.remove(name);
        }
    }

    for n in names.iter().zip(my_ticket.iter()) {
        eprintln!("{:?}", n);
    }

    println!(
        "{}",
        names
            .iter()
            .map(|m| (my_ticket[m.0], *m.1.iter().next().unwrap()))
            .filter(|(_, name)| name.starts_with("departure"))
            .map(|(a, _)| a as u64)
            .inspect(|n| eprintln!("{}", n))
            .product::<u64>()
    );
}
