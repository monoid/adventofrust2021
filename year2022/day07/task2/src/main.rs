use std::io::{self, Read};

use lib::parse_commands;

const MAX_SIZE: u32 = 70000000 - 30000000;

fn main() {
    let mut stdin = io::stdin().lock();
    let mut data = vec![];
    stdin.read_to_end(&mut data).unwrap();

    let commands = parse_commands(&data).unwrap().1;
    let mut shell = lib::Shell::new();
    for cmd in commands {
        shell.execute(cmd);
    }
    let root = shell.into_root();
    let root_size = root.borrow().size();
    let required_size = root_size.checked_sub(MAX_SIZE).unwrap();

    let mut max = root_size;
    root.borrow().walk_size(&mut |size| {
        if (required_size <= size) && (size < max) {
            max = size;
        }
    });
    println!("{}", max);
}
