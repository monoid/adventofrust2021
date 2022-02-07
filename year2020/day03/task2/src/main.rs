fn main() {
    let map = lib::read_map().unwrap();

    let res = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|&(dx, dy)| lib::count_trees(&map, dx, dy))
        .product::<usize>();

    println!("{}", res);
}
