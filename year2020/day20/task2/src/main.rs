use std::io;

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let data = lib::read_tiles(stdin);
    let composed = lib::compose_tiles(&data);
    let mut merged = lib::merge(&composed);
    let patterns = lib::patterns();

    let he = merged.len();
    let wi = merged[0].len();

    for pat in patterns {
        let sy = pat.iter().map(|&(_, y)| y).max().unwrap();
        let sx = pat.iter().map(|&(x, _)| x).max().unwrap();

        for li in 0..(he - sy) {
            for po in 0..(wi - sx) {
                if pat.iter().all(|&(dx, dy)| {
                    let x = po + dx;
                    let y = li + dy;
                    merged[y][x] != b'.'
                }) {
                    for &(dx, dy) in &pat {
                        let x = po + dx;
                        let y = li + dy;
                        merged[y][x] = b'O';
                    }
                }
            }
        }
    }
    println!(
        "{}",
        merged.iter().flatten().filter(|&&b| b == b'#').count()
    );
}
