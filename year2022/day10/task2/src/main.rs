fn main() {
    let data = lib::load_data();
    let cpu = lib::Cpu::new();
    let states = cpu.execute(&data);

    for (cycle, value) in states {
        let offset = (cycle as isize + 39) % 40;

        let c = if (offset - value).abs() < 2 { '#' } else { '.' };
        print!("{}", c);
        if (offset + 1) % 40 == 0 {
            println!();
        }
    }
}
