use std::io::{self, BufRead as _};

#[derive(Clone, Copy, Debug)]
enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

#[derive(Clone, Copy, Debug, Default)]
struct WeAllLiveInASantasSubmarine {
    x: i32,
    y: i32,
    aim: i32,
}

impl WeAllLiveInASantasSubmarine {
    fn product(self) -> i32 {
        self.x * self.y
    }

    fn apply(self, cmd: Command) -> Self {
        match cmd {
            Command::Forward(val) => Self {
                x: self.x + val,
                y: self.y + val * self.aim,
                ..self
            },
            Command::Down(val) => Self {
                aim: self.aim + val,
                ..self
            },
            Command::Up(val) => Self {
                aim: self.aim - val,
                ..self
            },
        }
    }
}

fn parse_command<A: AsRef<str>>(str: A) -> Command {
    let mut split = str.as_ref().split_ascii_whitespace();
    let cmd = split.next().unwrap();
    let val = split.next().unwrap().parse().expect("Invalid num");
    if cmd == "forward" {
        Command::Forward(val)
    } else if cmd == "down" {
        Command::Down(val)
    } else if cmd == "up" {
        Command::Up(val)
    } else {
        panic!("Unknown command {:?}", cmd)
    }
}

fn main() {
    println!(
        "{}",
        io::stdin()
            .lock()
            .lines()
            .map(Result::unwrap)
            .map(parse_command)
            .fold(WeAllLiveInASantasSubmarine::default(), |acc, cmd| acc
                .apply(cmd))
            .product()
    );
}
