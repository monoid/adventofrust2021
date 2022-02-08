use std::io;

pub fn read_nums<R: io::BufRead>(inp: R) -> impl Iterator<Item = u64> {
    inp.lines().map(|l| l.unwrap().parse().unwrap())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
