use std::collections::HashMap;

fn validate(rec: &&HashMap<String, String>) -> bool {
    lib::REQUIRED
        .iter()
        .all(|&(key, flt)| rec.get(key).map(|val| flt(val)).unwrap_or(false))
}

fn main() {
    let data = lib::read_data();
    println!("{}", data.iter().filter(validate).count());
}
