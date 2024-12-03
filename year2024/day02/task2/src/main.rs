fn main() {
    let mut cnt = 0;
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let nums: Vec<_> = lib::parse(&line).collect();
        cnt += validate2(&nums) as i32;
    }
    println!("{cnt}");
}

fn validate2(nums: &[i32]) -> bool {
    if lib::validate(nums.iter().cloned()) {
        return true;
    }
    // BRUTEFORCE!
    (0..nums.len()).any(|excluded| {
        lib::validate(nums[0..excluded].iter().cloned().chain(nums[excluded+1..].iter().cloned()))
    })
}
