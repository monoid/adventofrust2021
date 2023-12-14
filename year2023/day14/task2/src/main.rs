use std::collections::hash_map::Entry;
use std::collections::HashMap;

const N: usize = 1000000000;

fn main() {
    let mut data = lib::read_data();
    let mut history = HashMap::<_, (usize, usize)>::new();
    let mut load_by_id = HashMap::new();

    for i in 1..N {
        lib::tilt_north(&mut data);
        lib::tilt_west(&mut data);
        lib::tilt_south(&mut data);
        lib::tilt_east(&mut data);

        let load = lib::calc_load(&data);

        load_by_id.insert(i, load);

        match history.entry(data.clone()) {
            Entry::Occupied(prev) => {
                let prev_id = prev.get().0;
                let orbit_len = i - prev_id;

                let target_id = ((N - prev_id) % orbit_len) + prev_id;
                println!("{}", load_by_id.get(&target_id).unwrap());

                break;
            }
            Entry::Vacant(place) => {
                place.insert((i, load));
            }
        }
    }
}
