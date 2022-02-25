use std::io;

pub fn read_data<R: io::BufRead>(inp: R) -> (u32, u32) {
    let mut lines = inp.lines();
    (
        lines.next().unwrap().unwrap().parse().unwrap(),
        lines.next().unwrap().unwrap().parse().unwrap(),
    )
}

const SUBJ: u32 = 7;
const MOD: u32 = 20201227;

pub fn find_key(target: u32) -> usize {
    let mut val = 1;
    for i in 0.. {
        if val == target {
            return i;
        }
        val = (val * SUBJ) % MOD;
    }
    unreachable!()
}

pub fn apply_rounds(subj: u32, rounds: usize) -> u32 {
    let mut val = 1;
    let subj = subj as u64;
    for _ in 0..rounds {
        val = ((val as u64 * subj) % MOD as u64) as u32;
    }
    val
}
