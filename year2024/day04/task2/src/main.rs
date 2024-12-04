fn main() {
    let map = lib::Map::read(std::io::stdin());
    let mut cnt = 0;
    for y in 1..(map.height() - 1) {
        for x in 1..(map.width() - 1) {
            let m = &map.map;
            let c = m[y][x];
            if c == b'A' {
                let lt = m[y - 1][x - 1];
                let rt = m[y - 1][x + 1];

                let lb = m[y + 1][x - 1];
                let rb = m[y + 1][x + 1];

                if ((lt == b'M' && rb == b'S') || (lt == b'S' && rb == b'M'))
                    && ((rt == b'M' && lb == b'S') || (rt == b'S' && lb == b'M'))
                {
                    cnt += 1;
                }
            }
        }
    }

    println!("{cnt}");
}
