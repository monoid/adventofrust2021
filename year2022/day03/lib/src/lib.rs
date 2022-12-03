use std::io::{self, BufRead};

pub fn read_items() -> impl Iterator<Item = (String, String)> {
    let stdin = io::stdin();
    stdin.lock().lines().map(|r| {
        let line = r.unwrap();
        // We assume here that lines are ASCII, otherwise we will get incorrect len.
        let (p1, p2) = line.split_at(line.len() / 2);
        (p1.to_owned(), p2.to_owned())
    })
}

pub fn priority(c: char) -> u8 {
    match c {
        'a'..='z' => ((c as u8) - b'a') + 1,
        'A'..='Z' => ((c as u8) - b'A') + 27,
        _ => panic!("Unexpected character {:?}", c),
    }
}

#[cfg(test)]
mod tests {
    use crate::priority;

    #[test]
    fn test_priority() {
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('b'), 2);
        assert_eq!(priority('z'), 26);
        assert_eq!(priority('A'), 27);
        assert_eq!(priority('B'), 28);
        assert_eq!(priority('Z'), 52);
    }
}
