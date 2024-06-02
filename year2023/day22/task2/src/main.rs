fn main() {
    let mut data = lib::read_data();
    data.sort_by_key(|p| p.min_z());
    let mut map = lib::Map::new();

    for (slab_id, slab) in data.iter_mut().enumerate() {
        map.drop_slap(slab_id, slab);
    }

    // init dominator algorithm
    let slab_count = data.len();

    // this is one-pass dominator calculation alogorithm because slabs are topologically sorted
    for idx in 0..slab_count {
        let mut parent_dominators = data[idx]
            .supported_by
            .iter()
            .map(|idx| &data[*idx].dominators);
        let mut new_dominators = match parent_dominators.next() {
            Some(init) => parent_dominators.fold(init.clone(), |set, item| {
                set.intersection(item).cloned().collect()
            }),
            None => <_>::default(),
        };

        new_dominators.insert(idx);

        data[idx].dominators = new_dominators;
    }

    let answer = data
        .iter()
        .map(|slab| slab.dominators.len() - 1)
        .sum::<usize>();
    println!("{answer}");
}
