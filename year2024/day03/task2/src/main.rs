use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let commands = lib::extract2(&buf);
    let mut enabled = true;
    let mut sum = 0;
    for ex in commands {
        use lib::Extracted::*;
        match ex {
            Mul(a, b) => {
                if enabled {
                    sum += a * b;
                }
            }
            Do => enabled = true,
            Dont => enabled = false,
        }
    }
    println!("{sum}");
}
