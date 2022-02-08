use std::io;

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let mut nums = lib::read_nums(stdin).collect::<Vec<_>>();
    nums.push(0); // Source
    nums.sort_unstable();

    let mut cnt1 = 0;
    let mut cnt3 = 1;

    for w in nums.windows(2) {
        let d = w[1] - w[0];
        assert!((1..=3).contains(&d));

        match d {
            1 => cnt1 += 1,
            3 => cnt3 += 1,
            _ => {}
        }
    }

    println!("{}", cnt1 * cnt3);
}
