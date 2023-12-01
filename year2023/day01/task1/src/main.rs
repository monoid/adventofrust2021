fn main() {
    let result: u32 = std::io::stdin()
        .lines()
        .filter(|l| !l.as_ref().unwrap().is_empty())
        .map(|l| parse_input(&l.unwrap()))
        .sum();
    eprintln!("{}", result);
}

fn parse_input(inp: &str) -> u32 {
    let digits = inp.chars().filter(char::is_ascii_digit);
    let first = digits.clone().next().unwrap() as u8 - b'0';
    let last = digits.last().unwrap_or_else(|| panic!("{:?}", inp)) as u8 - b'0';

    (10 * first + last) as _
}
