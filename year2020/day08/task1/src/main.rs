use std::io;

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let prog = lib::read_prog(stdin);
    let mut cpu = lib::Cpu::default();
    let mut trace = vec![false; prog.len()];

    while !trace[cpu.pc] {
        trace[cpu.pc] = true;
        cpu.exec_cmd(&prog);
    }

    println!("{}", cpu.acc);
}
