use std::io;

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let mut nums = lib::read_nums(stdin).collect::<Vec<_>>();
    nums.push(0); // Source
    nums.sort_unstable();
    let mut counts = Vec::with_capacity(nums.len());
    assert!(nums[0] == 0);
    counts.push(1);

    for j in 1..nums.len() {
        let target = nums[j];
        counts.push(
            counts
                .iter()
                .zip(nums[0..j].iter())
                .rev()
                .take_while(|&(_, n)| n + 3 >= target)
                .map(|(count, _)| count)
                .sum::<u64>(),
        );
    }
    println!("{}", counts.last().unwrap());
}
