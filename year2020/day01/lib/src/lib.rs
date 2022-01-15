use std::io::{self, BufRead as _};

pub fn read_input() -> Vec<i32> {
    io::stdin().lock().lines().map(|s| s.unwrap().parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
