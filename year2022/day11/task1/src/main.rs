fn main() {
    let data = lib::read_data();

    for _round in 0..20 {
        for monkey_id in 0..(data.len() as i32) {
            // It is assumed that monkeys are correctly numbered from 0 to N.
            let monkey_ref = data.get(&monkey_id).unwrap();
            let mut monkey = monkey_ref.borrow_mut();

            monkey.do_turn1(&data);
        }
    }

    let mut counts: Vec<_> = data
        .values()
        .map(|monkey_ref| monkey_ref.borrow().counter)
        .collect();
    counts.sort_unstable_by_key(|x| usize::MAX - x);
    println!("{}", counts[0] * counts[1]);
}
