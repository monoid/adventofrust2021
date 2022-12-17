fn main() {
    let map = lib::read_map();
    println!("{}", lib::count_all_visibles(&map));
}
