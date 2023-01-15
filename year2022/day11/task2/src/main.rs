fn main() {
    let data = lib::read_data();

    let lcm = lib::lcm(
        &(data
            .values()
            .map(|m| m.borrow().decl.test.divisible)
            .collect::<Vec<_>>()),
    );

    for _round in 0..10000 {
        for monkey_id in 0..(data.len() as i32) {
            // It is assumed that monkeys are correctly numbered from 0 to N.
            let monkey_ref = data.get(&monkey_id).unwrap();
            let mut monkey = monkey_ref.borrow_mut();

            monkey.do_turn2(&data, lcm);
        }
    }

    let mut counts: Vec<_> = data
        .values()
        .map(|monkey_ref| monkey_ref.borrow().counter)
        .collect();
    counts.sort_unstable_by_key(|x| usize::MAX - x);
    println!("{}", counts[0] * counts[1]);
}
