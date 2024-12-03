pub fn parse(data: &str) -> impl Iterator<Item = i32> + '_ {
    data.split_ascii_whitespace()
        .map(|t| t.parse::<i32>().unwrap())
}

pub fn validate(nums: impl Iterator<Item = i32>) -> bool {
    use std::cmp::Ordering::*;

    let mut nums = nums.peekable();
    let a = nums.next().unwrap();
    let b = nums.peek().unwrap();

    match a.cmp(&b) {
        Less => check_up(a, nums),
        Greater => check_down(a, nums),
        Equal => false,
    }
}

fn check_up<I: Iterator<Item = i32>>(mut prev: i32, it: I) -> bool {
    for next in it {
        if prev >= next || !delta_ok(prev, next) {
            return false;
        }
        prev = next;
    }
    return true;
}

fn check_down<I: Iterator<Item = i32>>(mut prev: i32, it: I) -> bool {
    for next in it {
        if prev <= next || !delta_ok(prev, next) {
            return false;
        }
        prev = next;
    }
    return true;
}

fn delta_ok(a: i32, b: i32) -> bool {
    let d = a.abs_diff(b);
    1 <= d && d <= 3
}
