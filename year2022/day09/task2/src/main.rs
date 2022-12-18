use std::collections::HashSet;

fn main() {
    let commands = lib::read_data();
    let mut rope = lib::Rope::new(10);
    let mut trace = HashSet::new();
    trace.insert(<_>::default());

    for cmd in commands {
        rope.execute_command(cmd, &mut trace);
    }
    println!("{}", trace.len());
}
