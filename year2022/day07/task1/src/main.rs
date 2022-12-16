use std::io::{self, Read};

use lib::parse_commands;

const MAX_SIZE: u32 = 100000;

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
    let mut sum = 0u32;
    root.borrow().walk_size(&mut |size| {
        if size <= MAX_SIZE {
            sum += size;
        }
    });
    println!("{}", sum);
}
