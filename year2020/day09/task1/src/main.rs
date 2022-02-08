use std::io;

fn is_sum(item: u64, prev: &[u64]) -> bool {
    for i in 0..prev.len() {
        for j in (i + 1)..prev.len() {
            if prev[i] + prev[j] == item {
                return true;
            }
        }
    }
    false
}

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let nums = lib::read_nums(stdin).collect::<Vec<_>>();
    let size = 25;
    for win in nums.windows(size + 1) {
        if !is_sum(win[size], &win[0..size]) {
            println!("{}", win[size]);
            break;
        }
    }
}
