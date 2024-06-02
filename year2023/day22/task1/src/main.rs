use std::collections::HashSet;

fn main() {
    let mut data = lib::read_data();
    data.sort_by_key(|p| p.min_z());
    let mut map = lib::Map::new();

    for (slab_id, slab) in data.iter_mut().enumerate() {
        map.drop_slap(slab_id, slab);
    }

    // Detect desintigration bricks.
    let mut des = HashSet::<lib::SlabId>::new();
    for slab in &data {
        if slab.supported_by.len() == 1 {
            des.extend(slab.supported_by.iter().cloned());
        }
    }

    println!("{}", data.len() - des.len());
}
