use std::collections::HashSet;

fn main() {
    let mut map = lib::Map::read(std::io::stdin().lock());
    let mut visited = lib::find_visited(&map);
    visited.remove(&(map.init_pos.0 as _, map.init_pos.1 as _));

    let mut cnt = 0;
    let mut guards = HashSet::new();

    for pos in visited {
        map.data[pos.1 as usize][pos.0 as usize] = true;

        if loops(&map, &mut guards) {
            cnt += 1;
        }

        map.data[pos.1 as usize][pos.0 as usize] = false;
        guards.clear();
    }
    println!("{}", cnt);
}

fn loops(map: &lib::Map, guards: &mut HashSet<lib::Guard>) -> bool {
    let mut guard = lib::Guard::new(map.init_pos, lib::Dir::Up);

    while guard.try_move(map).is_some() {
        if guards.contains(&guard) {
            return true;
        }
        guards.insert(guard);
    }
    false
}
