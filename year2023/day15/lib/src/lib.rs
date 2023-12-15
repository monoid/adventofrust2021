pub fn hash(data: &[u8]) -> u8 {
    data.iter()
        .cloned()
        .fold(0, |a, b| a.wrapping_add(b).wrapping_mul(17))
}

pub fn read_data_v1() -> Vec<String> {
    std::io::stdin()
        .lines()
        .flat_map(|line| {
            let line = line.unwrap();
            line.trim()
                .split(',')
                .map(ToOwned::to_owned)
                .collect::<Vec<_>>()
        })
        .collect()
}

#[derive(Debug)]
pub enum Command {
    Add(String, u32),
    Remove(String),
}

#[derive(Debug, Clone)]
pub struct Lens {
    pub label: String,
    pub power: u32,
}

fn parse_command(inp: &str) -> Command {
    if let Some((label, n)) = inp.split_once('=') {
        Command::Add(label.to_owned(), n.parse().unwrap())
    } else {
        assert!(inp.ends_with('-'));
        Command::Remove(inp.strip_suffix('-').unwrap_or(inp).to_owned())
    }
}

pub fn read_data_v2() -> Vec<Command> {
    std::io::stdin()
        .lines()
        .flat_map(|line| {
            let line = line.unwrap();
            line.trim()
                .split(',')
                .map(ToOwned::to_owned)
                .collect::<Vec<_>>()
        })
        .map(|cmd| parse_command(&cmd))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash(b"HASH"), 52);
    }
}
