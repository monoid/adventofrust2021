use std::io;

fn mutate_cmd(cmd: lib::Cmd) -> Option<lib::Cmd> {
    match cmd.op {
        lib::Op::Acc => None,
        lib::Op::Jmp => Some(lib::Cmd {
            op: lib::Op::Nop,
            arg: cmd.arg,
        }),
        lib::Op::Nop => Some(lib::Cmd {
            op: lib::Op::Jmp,
            arg: cmd.arg,
        }),
    }
}

fn mutations(prog: &[lib::Cmd]) -> Vec<Vec<lib::Cmd>> {
    let mut res = vec![];

    for (pos, &cmd) in prog.iter().enumerate() {
        if let Some(m) = mutate_cmd(cmd) {
            let mut mutated = prog.to_owned();
            mutated[pos] = m;
            res.push(mutated);
        }
    }
    res
}

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let prog = lib::read_prog(stdin);

    let mutateds = mutations(&prog);
    let mut cpus = vec![lib::Cpu::default(); mutateds.len()];

    loop {
        for (cpu, prog) in cpus.iter_mut().zip(mutateds.iter()) {
            if cpu.exec_cmd(prog) {
                println!("{}", cpu.acc);
                return;
            }
        }
    }
}
