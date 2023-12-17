fn main() {
    let map = lib::read_data();
    let init_pos: (lib::Pos, lib::Dir) = ((0, 0), lib::Dir::Right);

    let count = lib::find_energy(init_pos, map);

    println!("{count}");
}
