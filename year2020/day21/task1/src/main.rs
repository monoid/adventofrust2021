use std::{collections::HashSet, io};

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let data = lib::read_data(stdin);
    let prod_dict = lib::calc_product_dict(&data);
    let allerg = HashSet::<_>::from_iter(prod_dict.into_values());
    println!(
        "{}",
        data.iter()
            .flat_map(|(m, _)| m.iter())
            .filter(|&for_| !allerg.contains(for_))
            .count()
    );
}
