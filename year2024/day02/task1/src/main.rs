fn main() {
    let mut cnt = 0;
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let nums = lib::parse(&line);
        cnt += lib::validate(nums) as i32;
    }
    println!("{cnt}");
}
