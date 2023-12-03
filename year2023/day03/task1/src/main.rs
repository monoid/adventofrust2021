fn main() {
    let scene = lib::read_scene();

    let size = (scene[0].len(), scene.len());

    let mut acc = 0;

    for (y, line) in scene.iter().enumerate() {
        let mut it = line.as_str();
        let mut offset = 0;
        // working with bytes strings would be easier
        while let Some(start_x) = it.find(|c| ('0'..='9').contains(&c)) {
            // we've found a start of number; skip the garbage
            it = &it[start_x..];
            offset += start_x;

            let (num, rest) = match it.find(|c| !('0'..='9').contains(&c)) {
                Some(end_num) => it.split_at(end_num),
                None => (it, ""),
            };

            // handle it
            let val: u32 = num.parse().unwrap();
            let x1 = offset;
            let x2 = offset + num.len();

            'all: for x in x1..x2 {
                for (x, y) in lib::around((x, y), size) {
                    // ascii
                    let c = scene[y].as_bytes()[x];
                    if (c != b'.') && !(c.is_ascii_digit()) {
                        acc += val;
                        break 'all;
                    }
                }
            }

            it = rest;
            offset = x2;
        }
    }

    println!("{}", acc);
}
