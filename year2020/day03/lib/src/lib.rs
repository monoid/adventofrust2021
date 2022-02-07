use std::io::{self, BufRead};

pub fn read_map() -> Result<Vec<Vec<u8>>, io::Error> {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    stdin.lines().map(|r| r.map(String::into_bytes)).collect()
}

pub fn count_trees(map: &[Vec<u8>], dx: usize, dy: usize) -> usize {
    let mut pos = 0;
    let mut count = 0;

    for (i, l) in map.iter().enumerate() {
        if i % dy != 0 {
            continue;
        }
        if l[pos] == b'#' {
            count += 1;
        }
        pos += dx;
        pos %= l.len();
    }
    count
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
