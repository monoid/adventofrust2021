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

fn find_range(item: u64, input: &[u64]) -> Option<&[u64]> {
    for i in 0..input.len() {
        // Start from i...
        let mut sum = 0;
        for (j, v) in input[i..].iter().enumerate() {
            sum += v;
            match sum.cmp(&item) {
                std::cmp::Ordering::Equal => {
                    return Some(&input[i..=(i + j)]);
                }
                std::cmp::Ordering::Greater => {
                    break;
                }
                _ => {}
            }
        }
    }
    None
}

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let nums = lib::read_nums(stdin).collect::<Vec<_>>();
    let size = 25;
    for win in nums.windows(size + 1) {
        if !is_sum(win[size], &win[0..size]) {
            let range = find_range(win[size], &nums).unwrap();
            println!(
                "{}",
                range.iter().min().unwrap() + range.iter().max().unwrap()
            );
            break;
        }
    }
}
