use std::str::FromStr;

fn main() {
    let (path, labirinth) = lib::read_scene();

    let mut node = lib::Node::from_str("AAA").unwrap();
    let target = lib::Node::from_str("ZZZ").unwrap();

    let mut cnt = 1;

    for dir in path.iter().cloned().cycle() {
        let paths = labirinth.get(&node).unwrap();
        node = *dir.select(&paths);

        if node == target {
            break
        } else {
            cnt += 1;
        }
    }

    println!("{}", cnt);
}
