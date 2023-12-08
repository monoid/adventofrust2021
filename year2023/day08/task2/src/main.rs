fn main() {
    let (path, labirinth) = lib::read_scene();

    let initital_nodes: Vec<_> = labirinth
        .keys()
        .filter(|k| k.last_char() == 'A')
        .cloned()
        .collect();

    let mut lasts = vec![];

    for mut node in initital_nodes {
        let mut cnt: u64 = 1;
        let mut states = vec![(node, cnt, 0)];

        for dir in path.iter().cloned().cycle() {
            let paths = labirinth.get(&node).unwrap();
            node = *dir.select(&paths);

            let is_last = states.iter().any(|(n, _, _)| n == &node);
            if node.last_char() == 'Z' {
                let last_cnt = states.last().unwrap().1;
                states.push((node, cnt, cnt - last_cnt));
                if is_last {
                    lasts.push(cnt - last_cnt);
                }
            }
            if is_last {
                break;
            }
            cnt += 1;
        }
    }

    println!("{}", lcm_all(&lasts));
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd::binary_u64(a, b)
}

fn lcm_all(vals: &[u64]) -> u64 {
    vals.iter().cloned().reduce(lcm).unwrap()
}
