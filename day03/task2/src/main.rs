use std::io::{self, BufRead as _};

#[derive(Clone, Copy, Debug)]
enum Mode {
    Min,
    Max,
}

impl Mode {
    fn cmp(self, a: usize, b: usize) -> bool {
        match self {
            Mode::Min => a <= b,
            Mode::Max => a > b,
        }
    }
}

fn select(mut nums: Vec<u32>, len: usize, mode: Mode) -> u32 {
    assert!(nums.len() > 0);

    for idx in (0..len).rev() {
        if nums.len() == 1 {
            break;
        }

        let zero_cnt = nums
            .iter()
            .map(|s| ((s >> idx) & 1) == 0)
            .filter(|b| *b)
            .count();

        let filter_flag = mode.cmp(zero_cnt, nums.len() / 2);

        // One could partition instead...
        nums = nums
            .into_iter()
            .filter(|n| (((n >> idx) & 1) == 0) == filter_flag)
            .collect();
    }
    // It is possible that few elements are left in the nums; however, they have
    // to be equal in this case.
    nums[0]
}

fn main() {
    let lines: Vec<_> = io::stdin()
        .lock()
        .lines()
        .collect::<Result<_, _>>()
        .unwrap();
    let len = lines.iter().map(String::len).max().unwrap();
    let nums: Vec<_> = lines
        .iter()
        .map(|s| u32::from_str_radix(s, 2).unwrap())
        .collect();

    println!(
        "{}",
        select(nums.clone(), len, Mode::Max) * select(nums, len, Mode::Min)
    );
}
