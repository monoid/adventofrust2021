use std::io;

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let data = lib::read_data(stdin);
    let prod_dict = lib::calc_product_dict(&data);
    let mut pairs: Vec<_> = prod_dict.into_iter().collect();
    pairs.sort_unstable();
    for (_, n) in pairs {
        print!("{},", n);
    }
    println!();
}
