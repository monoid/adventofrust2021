use std::io::{self, BufRead};

pub fn seats() -> Vec<u16> {
    let stdin = io::stdin();
    let stdin = stdin.lock().lines();

    stdin
        .map(|s| {
            let s = s.unwrap();
            u16::from_str_radix(
                &s.chars()
                    .map(|c| if c == 'B' || c == 'R' { '1' } else { '0' })
                    .collect::<String>(),
                2,
            )
            .unwrap()
        })
        .collect()
}
