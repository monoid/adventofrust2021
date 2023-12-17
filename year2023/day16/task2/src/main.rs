fn main() {
    let map = lib::read_data();

    let height = map.cells.len() as isize;
    let width = map.cells[0].len() as isize;

    let left_edge = (0..height).map(|y| ((0, y), lib::Dir::Right));
    let top_edge = (0..width).map(|x| ((x, 0), lib::Dir::Down));
    let right_edge = (0..height).map(|y| ((width - 1, y), lib::Dir::Left));
    let bottom_edge = (0..width).map(|x| ((x, height - 1), lib::Dir::Up));

    let max_count = left_edge
        .chain(top_edge)
        .chain(right_edge)
        .chain(bottom_edge)
        .map(|init| lib::find_energy(init, map.clone()))
        .max()
        .unwrap();

    println!("{max_count}");
}
