fn main() {
    let map = lib::Map::read(std::io::stdin());
    let mut queue = vec![]; // Well, this is a stack, but order doesn't matter.
    let first = lib::TARGET[0];

    let mut count = 0;

    for y in 0..map.height() {
        for x in 0..map.width() {
            if map.map[y][x] == first {
                for (dir, _) in map.around((x, y)) {
                    queue.push(lib::Match {
                        left: lib::TARGET.len() - 1,
                        pos: (x, y),
                        dir,
                    });
                    count += handle_queue(&mut queue, &map, lib::TARGET);
                }
            }
        }
    }

    println!("{count}");
}

fn handle_queue(queue: &mut Vec<lib::Match>, map: &lib::Map, target: &[lib::Cell]) -> u32 {
    let mut cnt = 0;

    while let Some(m) = queue.pop() {
        match m.advance(map, target) {
            Some(more) => queue.extend(more.filter(|n| n.dir == m.dir)),
            None => cnt += 1,
        }
    }

    cnt
}
