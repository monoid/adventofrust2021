fn main() {
    let mut data = lib::read_data();

    lib::tilt_north(&mut data);

    let load = lib::calc_load(&data);

    println!("{load}");
}
