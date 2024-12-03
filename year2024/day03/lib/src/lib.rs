pub fn extract1(input: &str) -> Vec<(u32, u32)> {
    let re = regex::Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    re.captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [a, b])| (a.parse().unwrap(), b.parse().unwrap()))
        .collect()
}

pub enum Extracted {
    Mul(u32, u32),
    Do,
    Dont,
}
pub fn extract2(input: &str) -> Vec<Extracted> {
    let re = regex::Regex::new(r"mul\(([0-9]+),([0-9]+)\)|do\(\)|don't\(\)").unwrap();
    re.captures_iter(input)
        .map(|c| {
            let whole = &c[0];
            if whole.starts_with("mul(") {
                Extracted::Mul(c[1].parse().unwrap(), c[2].parse().unwrap())
            } else if whole == "do()" {
                Extracted::Do
            } else if whole == "don't()" {
                Extracted::Dont
            } else {
                unreachable!()
            }

        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let items = extract1("amul(42,8)mul");
        assert_eq!(items, vec![(42, 8)]);
    }
}
