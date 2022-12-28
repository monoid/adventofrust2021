fn main() {
    let data = lib::load_data();
    let cpu = lib::Cpu::new();
    let states = cpu.execute(&data);
    let res = states
        .filter_map(|(cycle, value)| {
            if (cycle + 20) % 40 == 0 {
                Some((cycle as isize) * value)
            } else {
                None
            }
        })
        .sum::<isize>();
    println!("{}", res);
}
