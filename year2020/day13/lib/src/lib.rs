use std::io;

pub fn parse_input<R: io::BufRead>(inp: &mut R) -> (usize, Vec<Option<usize>>) {
    let mut ts_line = String::new();
    inp.read_line(&mut ts_line).unwrap();
    let mut sc_line = String::new();
    inp.read_line(&mut sc_line).unwrap();

    let ts = ts_line.trim().parse().unwrap();
    let data = sc_line
        .trim()
        .split(',')
        .map(|tok| {
            if tok == "x" {
                None
            } else {
                Some(tok.parse().unwrap())
            }
        })
        .collect();
    (ts, data)
}
